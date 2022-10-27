use core::fmt::{Debug, Formatter};
use std::alloc::{alloc, alloc_zeroed, Allocator, AllocError, dealloc, Global, Layout, realloc};

use std::intrinsics::{size_of};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit, replace, size_of_val, take};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};
use std::collections::HashMap;
use std::ptr::{addr_of_mut, NonNull, null_mut, Unique};
use fxhash::FxHashMap;
use num_format::{Locale, ToFormattedString};
use crate::leblanc::configuration::{ALLOW_HEAP_REALLOC, ALLOW_HEAP_REALLOC_FOR_WILD, HMX_MB};
use crate::leblanc::include::lib::leblanc_colored::{Color, colorize_str};
use crate::leblanc::rustblanc::better_static::BetterStatic;
use crate::leblanc::rustblanc::memory::megabyte::Megabyte;
use crate::leblanc::rustblanc::memory::{MemoryFootprint, MemoryUnit};
use crate::leblanc::rustblanc::memory::pointer::WildPointer;
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;
use crate::unsafe_vec;

static mut NULL_OBJ: BetterStatic<Unique<u8>> = BetterStatic::new(HeapObject::<u8>::null_ptr);


trait RawHeap: Send {
    fn free(&mut self, ptr: WildPointer);
}

pub struct TypedHeap<T> {
    root: Unique<TypedHeap<T>>,
    backend: Unique<HeapObject<T>>,
    allocated: usize,
    capacity: usize,
    virtual_size: usize,
    available: UnsafeVec<Unique<HeapObject<T>>>,
    free_method: unsafe fn(&mut TypedHeap<T>, *mut HeapObject<T>),

    /// # Mode
    /// Mode determines the behavior of this heap when acquiring allocated objects:
    /// * **0** | Allocation / De-allocation behaves as normal
    /// * **1** | Allocations are disabled, de-allocation checks if whole memory
    /// block is free. If true, block will destroy itself.
    mode: u8
}

#[cfg(target_env = "msvc")]
impl<T: Default + Clone + Debug + 'static> Default for TypedHeap<T> {
    fn default() -> Self {
        TypedHeap::new(1)
    }
}


/// Default for Heap, jemalloc doesn't allow for 0 byte layouts so we need to have at least 1 capacity
#[cfg(not(target_env = "msvc"))]
impl<T: Default + Clone + Debug + 'static> Default for TypedHeap<T> {
    fn default() -> Self {
        TypedHeap::new(1)
    }
}


unsafe impl<T> Sync for TypedHeap<T> {}

unsafe impl<T> Sync for HeapObject<T>
where T: Sync {}

#[derive(Debug)]
pub struct HeapObject<T> {
    data: T,
    counter: usize,
    heap: Unique<dyn RawHeap>,
}


pub struct HeapRef<'a, T> {
    pointer: Unique<HeapObject<T>>,
    phantom: PhantomData<&'a mut T>,
}

impl<T: Debug> Debug for HeapRef<'static, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        unsafe {
            f.debug_struct("HeapRef")
                .field("pointer", &self.pointer)
                .field("data", self.pointer.as_ref())
                .finish()
        }
    }
}

impl<T: Default + Clone + Debug + 'static> TypedHeap<T> {
    #[inline(always)]
    pub fn new_bytes(capacity: Box<dyn MemoryUnit>) -> TypedHeap<T> {
        let element_capacity = (capacity.usize()) / size_of::<HeapObject<T>>();
        println!("Created Heap with size: {} and capacity for {} elements.", capacity, element_capacity.to_formatted_string(&Locale::en));
        TypedHeap::new(element_capacity)
    }


    #[inline(always)]
    pub fn new(capacity: usize) -> TypedHeap<T> {
        let heap_layout = Layout::new::<TypedHeap<T>>();

        let heap = unsafe { alloc_zeroed(heap_layout) } as *mut TypedHeap<T>;
        unsafe {
            (*heap).root = Unique::new_unchecked(heap);
            (*heap).available = UnsafeVec::new();
            (*heap).capacity = capacity;
            (*heap).virtual_size = capacity;
            let layout = Layout::array::<HeapObject<T>>(capacity).expect("Unable to allocate memory.");
            (*heap).backend = Unique::new_unchecked(alloc_zeroed(layout) as *mut HeapObject<T>);
            (*heap).mode = 0;
            (*heap).allocated = 0;
            (*heap).free_method = TypedHeap::smart_free;
            *Box::from_raw(heap)
        }
    }

    unsafe fn destruction(root: *mut TypedHeap<T>, block_start: *mut HeapObject<T>, allocations: usize, old_capacity: usize) -> &'static mut TypedHeap<T> {
        let heap_layout = Layout::new::<TypedHeap<T>>();
        let heap = alloc_zeroed(heap_layout) as *mut TypedHeap<T>;
        (*heap).root = Unique::new_unchecked(root);
        (*heap).available = UnsafeVec::new();
        (*heap).capacity = 0;
        (*heap).virtual_size = old_capacity;
        (*heap).backend = Unique::new_unchecked(block_start);
        (*heap).mode = 1;
        (*heap).allocated = allocations;
        (*heap).free_method = TypedHeap::destruct_free;
        Box::leak(Box::from_raw(heap))
    }

    #[inline(always)]
    pub fn alloc(&mut self, item: T) -> HeapRef<'static, T>
    where T: Send {
        self.alloc_with(|| item)
    }

    #[inline(always)]
    pub fn alloc_with<F>(&mut self, f: F) -> HeapRef<'static, T>
    where
        T: Send,
        F: FnOnce() -> T
    {
        if let Some(mut pointer) = self.available.pop_quick() {
            unsafe {
                let pointer = pointer.as_mut();
                pointer.data = f();
                pointer.counter += 1;
            }
            HeapRef { pointer, phantom: PhantomData }
        } else if self.allocated < self.capacity {
            unsafe {
                let target_ptr = self.backend.as_ptr().add(self.allocated);
                ptr::write(target_ptr, HeapObject::from_func(f, self as *mut TypedHeap<T>));
                let h_ref = HeapRef { pointer: Unique::new_unchecked(target_ptr), phantom: PhantomData };
                self.allocated += 1;
                h_ref
            }
        } else if ALLOW_HEAP_REALLOC {
            unsafe { self.expand_heap(); }
            self.alloc_with(f)
        } else {
            panic!("Maximum Heap Memory Size Exceeded")
        }
    }

    #[inline(always)]
    pub fn clone_heap_vec(&mut self, mut vec: &UnsafeVec<HeapRef<'_, T>>) -> UnsafeVec<HeapRef<'static, T>>
    where T: Send + 'static {
        let size = vec.len();
        let mut marker = 0;
        if self.allocated + size < self.capacity {
            let vec_ptr = vec.as_ptr();
            let mut new_vec = UnsafeVec::with_capacity(size);
            unsafe {
                while marker < size {
                    let target_ptr = if let Some(pointer) = self.available.pop_quick() {
                        pointer.as_ptr()
                    } else {
                        let ptr = self.backend.as_ptr().add(self.allocated);
                        self.allocated += 1;
                        ptr
                    };
                    let obj_ptr = (*vec_ptr.add(marker)).pointer.as_ptr();
                    ptr::copy_nonoverlapping(obj_ptr, target_ptr, 1);
                    (*target_ptr).counter = 1;
                    (*target_ptr).heap = Unique::new_unchecked(self as *mut TypedHeap<T>);
                    marker += 1;
                    new_vec.push_quick(HeapRef { pointer: Unique::new_unchecked(target_ptr), phantom: PhantomData });
                }
            }
            new_vec
        } else {
            unsafe { self.expand_heap() }
            self.clone_heap_vec(vec)
        }
    }

    unsafe fn expand_heap(&mut self)
    where T: Send {
        if self.mode == 1 {
            panic!("Attempted to alloc with a Heap under destruction.");
        }
        self.virtual_size += self.capacity;
        let footprint = Megabyte::from(self.virtual_size * size_of::<HeapObject<T>>());
        if footprint > HMX_MB {
            panic!("Maximum Heap Memory Size Exceeded ({} > {})", footprint, HMX_MB);
        }
        unsafe {
            println!("{}", colorize_str("REALLOCATING MEMORY", Color::Red));
            let new_layout = Layout::array::<HeapObject<T>>(self.capacity).expect("Unable to allocate memory.");
            let destruction_heap = TypedHeap::destruction(self as *mut TypedHeap<T>, self.backend.as_ptr(), self.allocated, self.capacity);
            let destruction_ptr = Unique::new_unchecked(destruction_heap as *mut TypedHeap<T>);

            let mut start = 0;
            while start < self.allocated {
                let backend_ptr = self.backend.as_ptr().add(start);
                (*backend_ptr).heap = destruction_ptr;
                start += 1;
            }
            self.allocated = 0;

            let new_ptr = alloc_zeroed(new_layout) as *mut HeapObject<T>;
            self.backend = Unique::new_unchecked(new_ptr);
        }
    }
}

impl<T> TypedHeap<T> {
    unsafe fn null() -> TypedHeap<T> {
        let heap_layout = Layout::new::<TypedHeap<T>>();
        let heap = unsafe { alloc_zeroed(heap_layout) } as *mut TypedHeap<T>;
        *Box::from_raw(heap)
    }

    #[inline(always)]
    unsafe fn smart_free(heap: &mut TypedHeap<T>, ptr: *mut HeapObject<T>) {
        heap.available.push_quick(Unique::new_unchecked(ptr));
    }

    #[inline(always)]
    unsafe fn destruct_free(heap: &mut TypedHeap<T>, _ptr: *mut HeapObject<T>) {
        heap.allocated -= 1;
        if heap.allocated == 0 {
            drop(replace(heap, TypedHeap::null()));
        }
    }
}

impl<T: Send> RawHeap for TypedHeap<T> {
    #[inline(always)]
    fn free(&mut self, ptr: WildPointer) {
        let f = self.free_method;
        unsafe { f(self, ptr.as_ptr()); }
    }
}

impl<T: Default> Clone for HeapObject<T> {
    fn clone(&self) -> Self {
        HeapObject {
            data: T::default(),
            counter: 0,
            heap: self.heap,
        }
    }
}

impl<T: Default + 'static> HeapObject<T>
where T: Send {
    fn safe_new(heap: *mut TypedHeap<T>) -> HeapObject<T> {
        unsafe {
            HeapObject {
                data: Default::default(),
                counter: 0,
                heap: Unique::new_unchecked(heap),
            }
        }
    }
}

impl<T> HeapObject<T> {
    #[inline(always)]
    fn new(heap: *mut TypedHeap<T>) -> HeapObject<T>
    where T: Send + 'static {
        let layout = Layout::new::<HeapObject<T>>();
        unsafe {
            let ho = alloc(layout) as *mut HeapObject<T>;
            (*ho).counter = 1;
            (*ho).heap = Unique::new_unchecked(heap);
            *Box::from_raw(ho)
        }
    }

    #[inline(always)]
    unsafe fn from(data: T, heap: *mut dyn RawHeap) -> HeapObject<T> {
        HeapObject {
            data,
            counter: 1,
            heap: Unique::new_unchecked(heap),
        }
    }

    #[inline(always)]
    unsafe fn from_func<F>(f: F, heap: *mut dyn RawHeap) -> HeapObject<T>
    where F: FnOnce() -> T {
        HeapObject {
            data: f(),
            counter: 1,
            heap: Unique::new_unchecked(heap),
        }
    }

    #[inline(always)]
    fn available(&mut self) {
        let p = WildPointer::new(self as *mut HeapObject<T>);
        unsafe { self.heap.as_mut().free(p); }
    }

    #[inline(always)]
    pub fn get_ptr(&self) -> *const T {
        ptr::addr_of!(self.data)
    }

    #[inline(always)]
    pub fn get_mut_ptr(&mut self) -> *mut T {
        ptr::addr_of_mut!(self.data)
    }

    #[inline(always)]
    pub fn get_mut_ptr_from_const(&self) -> *mut T {
        self.get_ptr() as *mut T
    }

    #[inline(always)]
    fn null_ptr() -> Unique<u8> {
        let layout = Layout::new::<HeapObject<T>>();
        unsafe {
            let ho = alloc(layout) as *mut HeapObject<T>;
            (*ho).counter = usize::MAX / 2;
            Unique::new_unchecked(ho as *mut u8)
        }
    }
}

impl<T: Default> Default for HeapObject<T> {
    #[inline(always)]
    fn default() -> Self {
        unsafe { *Box::from_raw(NULL_OBJ.as_ptr() as *mut HeapObject<T>) }
    }
}

impl<T> HeapRef<'_, T> {
    unsafe fn null() -> HeapRef<'static, T> {
        HeapRef {
            pointer: Unique::new_unchecked(NULL_OBJ.as_ptr() as *mut HeapObject<T>),
            phantom: Default::default()
        }
    }
}

impl<T> Deref for HeapRef<'_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.pointer.as_ref().data }
    }
}

impl<T> DerefMut for HeapRef<'_, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut self.pointer.as_mut().data }
    }
}

impl<T: PartialEq> PartialEq for HeapRef<'_, T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.pointer.as_ref().data.eq(&other.pointer.as_ref().data) }
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

impl<T: Default> Default for HeapRef<'_, T> {
    #[inline(always)]
    fn default() -> Self {
        unsafe {
            HeapRef {
                pointer: NULL_OBJ.access().cast(),
                phantom: Default::default()
            }
        }
    }
}


impl<T> Drop for HeapRef<'_, T> {
    #[inline(always)]
    fn drop(&mut self) {
        let deref = unsafe {&mut *self.pointer.as_ptr() };
        deref.counter = deref.counter.saturating_sub(1);
        if deref.counter == 0 {
            deref.available();
        }
    }
}


impl<T> Drop for TypedHeap<T> {
    fn drop(&mut self) {
        if self.mode == 0 {
            println!("Dropping Heap (End of Execution)");
            println!("I currently have {} objects allocated", self.allocated);
            println!("And {} free-pointers", self.available.len());
        } else {
            let old_layout = Layout::array::<HeapObject<T>>(self.virtual_size).expect("Couldn't build layout");
            unsafe {
                dealloc(self.backend.as_ptr() as *mut u8, old_layout);
                self.root.as_mut().virtual_size -= self.virtual_size;
            }
        }
    }
}

pub struct WildHeap {
    root: Unique<WildHeap>,
    backend: WildPointer,
    available: FxHashMap<Layout, UnsafeVec<WildPointer>>,
    heap_size: usize,
    max_size: usize,
    virtual_size: usize,
    mode: usize,
    free_method: unsafe fn(&mut WildHeap, WildPointer),
}

impl RawHeap for WildHeap {
    #[inline(always)]
    fn free(&mut self, ptr: WildPointer) {
        let f = self.free_method;
        unsafe { f(self, ptr); }
    }
}

impl WildHeap {
    unsafe fn null() -> WildHeap {
        let heap_layout = Layout::new::<WildHeap>();
        let heap = unsafe { alloc_zeroed(heap_layout) } as *mut WildHeap;
        *Box::from_raw(heap)
    }

    #[inline(always)]
    pub fn new(capacity: Box<dyn MemoryUnit>) -> WildHeap {
        let heap_layout = Layout::new::<WildHeap>();
        let heap = unsafe { alloc_zeroed(heap_layout) } as *mut WildHeap;
        unsafe {
            (*heap).root = Unique::new_unchecked(heap);
            (*heap).available = FxHashMap::default();
            (*heap).heap_size = 0;
            (*heap).virtual_size = 0;
            (*heap).max_size = capacity.usize();
            let layout = Layout::from_size_align(capacity.usize(), 8).expect("Unable to create layout");
            (*heap).backend = WildPointer::new(alloc_zeroed(layout));
            (*heap).mode = 0;
            (*heap).free_method = WildHeap::smart_free;
            *Box::from_raw(heap)
        }
    }

    unsafe fn destruction(root: *mut WildHeap, block_start: WildPointer, max_size: usize, old_size: usize) -> &'static mut WildHeap {
        let heap_layout = Layout::new::<WildHeap>();
        let heap = alloc_zeroed(heap_layout) as *mut WildHeap;
        (*heap).root = Unique::new_unchecked(root);
        (*heap).max_size = 0;
        (*heap).virtual_size = max_size;
        (*heap).heap_size = old_size;
        (*heap).backend = block_start;
        (*heap).mode = 1;
        (*heap).free_method = WildHeap::destruct_free;
        Box::leak(Box::from_raw(heap))
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, obj: T) -> HeapRef<'static, T>
    where T: Send
    {
        self.alloc_with(|| obj)
    }

    #[inline(always)]
    pub fn alloc_with<F, T>(&mut self, f: F) -> HeapRef<'static, T>
    where
        T: Send,
        F: FnOnce() -> T
    {
        let layout = Layout::new::<HeapObject<T>>();
        if let Some(free_pointers) = self.available.get_mut(&layout) {
            if let Some(pointer) = free_pointers.pop_quick() {
                let type_pointer = pointer.as_ptr() as *mut HeapObject<T>;
                unsafe {
                    let pointer =  &mut *type_pointer;
                    pointer.data = f();
                    pointer.counter += 1;
                    return HeapRef { pointer: Unique::new_unchecked(type_pointer), phantom: PhantomData };
                }
            }
        }
        if self.heap_size + layout.size() <= self.max_size {
            unsafe {
                let target_ptr = self.backend.as_ptr::<HeapObject<T>>().byte_add(self.heap_size) as *mut HeapObject<T>;
                ptr::write(target_ptr, HeapObject::from(f(), self as *mut WildHeap));
                let h_ref = HeapRef { pointer: Unique::new_unchecked(target_ptr), phantom: PhantomData };
                self.heap_size += layout.size();
                return h_ref
            }
        } else if ALLOW_HEAP_REALLOC_FOR_WILD {
            panic!("Maximum Heap Memory Size Exceeded")
            /*if self.mode == 1 {
                panic!("Attempted to alloc with a Heap under destruction.");
            }
            self.virtual_size += self.max_size;
            let footprint = Megabyte::from(self.virtual_size);
            if footprint > HMX_MB {
                panic!("Maximum Heap Memory Size Exceeded ({} > {})", footprint, HMX_MB);
            }
            unsafe {
                println!("{}", colorize_str("REALLOCATING MEMORY", Color::Red));
                let new_layout = Layout::from_size_align(self.max_size, 8).expect("Unable to create layout");
                let destruction_heap = WildHeap::destruction(self as *mut WildHeap, self.backend, self.max_size, self.heap_size);
                let destruction_ptr = Unique::new_unchecked(destruction_heap as *mut WildHeap);

                let mut start = 0;
                while start < self.allocated {
                    let backend_ptr = self.backend.as_ptr().add(start);
                    (*backend_ptr).heap = destruction_ptr;
                    start += 1;
                }
                self.allocated = 0;

                let new_ptr = alloc_zeroed(new_layout) as *mut HeapObject<T>;
                self.backend = Unique::new_unchecked(new_ptr);
            }
            self.alloc_with(f)*/
        } else {
            panic!("Maximum Heap Memory Size Exceeded")
        }
        unsafe { return HeapRef::null(); }
    }

    #[inline(always)]
    unsafe fn smart_free(heap: &mut WildHeap, ptr: WildPointer) {
        let layout = ptr.layout();
        if let Some(pointers) = heap.available.get_mut(&layout) {
            return pointers.push_quick(ptr);
        }
        heap.available.insert(layout, unsafe_vec![ptr]);
    }

    #[inline(always)]
    unsafe fn destruct_free(heap: &mut WildHeap, ptr: WildPointer) {
        heap.heap_size = heap.heap_size.saturating_sub(ptr.layout().size());
        if heap.heap_size == 0 {
            drop(replace(heap, WildHeap::null()));
        }
    }

    #[inline(always)]
    pub fn deep_clone<T: Clone + Send>(&mut self, r: &HeapRef<'_, T>) -> HeapRef<'static, T> {
        unsafe { self.alloc_with(|| r.pointer.as_ref().data.clone()) }
    }
}

impl<T> Debug for TypedHeap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WildHeap")
            .field("backend", &self.backend)
            .field("available", &self.available)
            .field("allocated", &self.allocated)
            .field("capacity", &self.capacity)
            .field("virtual_size", &self.virtual_size)
            .field("mode", &self.mode)
            .finish()
    }
}

impl Debug for WildHeap {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WildHeap")
            .field("backend", &self.backend)
            .field("available", &self.available)
            .field("heap_size", &self.heap_size)
            .field("max_size", &self.max_size)
            .field("virtual_size", &self.virtual_size)
            .field("mode", &self.mode)
            .finish()
    }
}