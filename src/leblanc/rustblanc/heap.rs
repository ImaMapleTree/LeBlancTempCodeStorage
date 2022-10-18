use core::fmt::Debug;
use std::alloc::{alloc, alloc_zeroed, Layout};
use std::collections::VecDeque;
use std::intrinsics::{size_of, size_of_val};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{null_mut, Unique};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::leblanc::core::leblanc_object::LeBlancObject;


#[derive(Debug)]
pub struct Heap<T> {
    backend: Unique<HeapObject<T>>,
    allocated: usize,
    capacity: usize,
    available: Vec<Unique<HeapObject<T>>>,
}


unsafe impl<T> Sync for Heap<T> {}

unsafe impl<T> Sync for HeapObject<T>
where T: Sync {}

#[derive(Debug)]
pub struct HeapObject<T> {
    data: T,
    counter: isize,
    heap: Unique<Heap<T>>
}

#[derive(Debug)]
pub struct HeapRef<'a, T> {
    pointer: Unique<HeapObject<T>>,
    phantom: PhantomData<&'a mut T>,
}

impl<T: Default + Clone + Debug> Heap<T> {
    #[inline(always)]
    pub fn new_bytes(capacity: usize) -> Heap<T> {
        let element_capacity = (capacity * 1000) / size_of::<HeapObject<T>>();
        Heap::new(element_capacity)
    }

    #[inline(always)]
    pub fn new(capacity: usize) -> Heap<T> {
        let heap_layout = Layout::from_size_align(size_of::<Heap<T>>(), 1).expect("Unable to allocate aligned memory");
        let mut heap = unsafe { alloc_zeroed(heap_layout) } as *mut Heap<T>;
        unsafe {
            (*heap).available = Vec::new();
            (*heap).capacity = capacity;
            let layout = Layout::array::<HeapObject<T>>(capacity).expect("Unable to allocate memory.");
            (*heap).backend = Unique::new_unchecked(alloc_zeroed(layout) as *mut HeapObject<T>);
            let raw = *Box::from_raw(heap);
            raw
        }
    }

    #[inline(always)]
    pub fn alloc(&mut self, item: T) -> HeapRef<'static, T> {
        self.alloc_with(|| item)
    }

    #[inline(always)]
    pub fn alloc_with<F>(&mut self, f: F) -> HeapRef<'static, T>
    where
        F: FnOnce() -> T
    {
        if let Some(mut pointer) = self.available.pop() {
            unsafe { pointer.as_mut().rewrite(f()); }
            HeapRef { pointer, phantom: PhantomData }
        } else if self.allocated < self.capacity {
            let obj = unsafe { HeapObject::from(f(), self as *mut Heap<T>) };
            unsafe { self.backend.as_ptr().write(obj) }
            let heap_ref = HeapRef { pointer: self.backend, phantom: PhantomData};
            self.backend = unsafe { Unique::new_unchecked( self.backend.as_ptr().add(1)) };
            self.allocated += 1;
            heap_ref
        } else {
            todo!()
        }
    }
}

















impl<T: Default> Clone for HeapObject<T> {
    fn clone(&self) -> Self {
        HeapObject {
            data: T::default(),
            counter: 0,
            heap: self.heap
        }
    }
}

impl<T: Default> HeapObject<T> {
    fn safe_new(heap: *mut Heap<T>) -> HeapObject<T> {
        unsafe {
            HeapObject {
                data: Default::default(),
                counter: 0,
                heap: Unique::new_unchecked(heap)
            }
        }
    }
}

impl<T> HeapObject<T> {
    #[inline(always)]
    fn new(heap: *mut Heap<T>) -> HeapObject<T> {
        let layout = Layout::new::<HeapObject<T>>();
        unsafe {
            let ho = alloc(layout) as *mut HeapObject<T>;
            (*ho).counter = 1;
            (*ho).heap = Unique::new_unchecked(heap);
            *Box::from_raw(ho)
        }
    }

    #[inline(always)]
    unsafe fn from(data: T, heap: *mut Heap<T>) -> HeapObject<T> {
        HeapObject {
            data,
            counter: 1,
            heap: Unique::new_unchecked(heap)
        }
    }

    #[inline(always)]
    fn available(&mut self) {
        unsafe {
            let p = Unique::new_unchecked(self as *mut HeapObject<T>);
            self.heap.as_mut().available.push(p)
        }
    }

    #[inline(always)]
    pub fn rewrite(&mut self, data: T) {
        self.data = data;
        self.counter += 1;
    }

    pub fn get_ptr(&self) -> *const T {
        &self.data as *const T
    }

    pub fn get_mut_ptr(&mut self) -> *mut T {
        &mut self.data as *mut T
    }

    pub fn get_mut_ptr_from_const(&self) -> *mut T {
        self.get_ptr() as *mut T
    }

    /*pub fn get_counter(&mut self) -> *mut AtomicUsize {
        &mut self.counter as *mut AtomicUsize
    }*/
}

impl<T> Deref for HeapRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.pointer.as_ref().data }
    }
}

impl<T: PartialEq> PartialEq for HeapRef<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.pointer.as_ref().data.eq(&other.pointer.as_ref().data) }
    }
}

/*impl<T> DerefMut for HeapRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.pointer).data }
    }
}

impl<T: Default + Clone> Deref for HeapRef<'_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.pointer).data }
    }
}*/

impl<T: Default + Clone> DerefMut for HeapRef<'_, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut self.pointer.as_mut().data }
    }
}

impl <T> Clone for HeapRef<'_, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {(*self.pointer.as_ptr()).counter += 1};
        HeapRef {
            pointer: self.pointer,
            phantom: PhantomData
        }
    }
}

impl<T> Default for HeapRef<'_, T> {
    #[inline(always)]
    fn default() -> Self {
        unsafe {
            HeapRef {
                pointer: Unique::new_unchecked(null_mut()),
                phantom: Default::default()
            }
        }
    }
}

impl<T> Drop for HeapRef<'_, T> {
    #[inline(always)]
    fn drop(&mut self) {
        let deref = unsafe { self.pointer.as_mut() };
        deref.counter -= 1;
        if deref.counter <= 0 {
            deref.available();
        }
    }
}