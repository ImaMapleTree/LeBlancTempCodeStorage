use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::mem::replace;
use std::{mem, ptr};
use std::ptr::null_mut;
use crate::arch::ArchSize;

pub struct SimpleVec<T> {
    ptr: *mut T,
    pub(crate) len: ArchSize,
    pub(crate) cap: ArchSize
}

impl<T> SimpleVec<T> {
    pub fn new() -> SimpleVec<T> {
        SimpleVec {
            ptr: null_mut(),
            len: 0,
            cap: 0
        }
    }

    pub fn double(&mut self) {
        let current_layout = Layout::array::<T>(self.cap).unwrap();
        self.cap *= 2;
        let new_layout = Layout::array::<T>(self.cap).unwrap();
        let new_ptr = unsafe { alloc_zeroed(new_layout) as *mut T};
        unsafe { ptr::copy_nonoverlapping(self.ptr, new_ptr, current_layout.size()) };
        let old_ptr = replace(&mut self.ptr, new_ptr);
        unsafe { dealloc(old_ptr as *mut u8, current_layout); }
    }

    pub fn with_capacity(capacity: usize) -> SimpleVec<T> {
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) } as *mut T;
        SimpleVec {
            ptr,
            len: 0,
            cap: capacity
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.double()
        }
        unsafe { *self.ptr.add(self.len) = value; }
        self.len += 1;
    }

    pub fn pop(&mut self) -> T {
        self.len -= 1;
        unsafe { ptr::read(self.ptr.add(self.len)) }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        let other_len = self.len - at;
        let mut other = SimpleVec::with_capacity(other_len);

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.len = at;
            other.len = other_len;

            ptr::copy_nonoverlapping(self.ptr.add(at), other.ptr, other.len);
        }
        other
    }

    pub fn split_off_as_vec(&mut self, at: usize, max_capacity: usize) -> Vec<T> {
        let other_len = self.len - at;
        let mut other = Vec::with_capacity(max_capacity);

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.len = at;
            other.set_len(other_len);

            ptr::copy_nonoverlapping(self.ptr.add(at), other.as_mut_ptr(), other.len());
        }
        other
    }
}

impl<T> Default for SimpleVec<T> {
    fn default() -> Self {
        Self::new()
    }
}