use std::alloc::Layout;
use std::mem;
use std::ptr::null_mut;

unsafe impl Send for WildPointer {}
unsafe impl Sync for WildPointer {}

#[derive(Clone, Copy, Debug)]
pub struct WildPointer {
    raw: *mut u8,
    layout: Layout
}

impl WildPointer {
    pub const fn new<T>(ptr: *mut T) -> WildPointer {
        let layout = unsafe {
            let (size, align) = (mem::size_of_val_raw(ptr), mem::align_of_val_raw(ptr));
            Layout::from_size_align_unchecked(size, align)
        };
        WildPointer { raw: ptr as *mut u8, layout }
    }

    pub const fn as_ptr<T>(&self) -> *mut T {
        self.raw as *mut T
    }

    pub const fn layout (&self) -> Layout {
        self.layout
    }
}

impl<T> AsMut<T> for WildPointer {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.raw as *mut T) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SyncPointer<T> {
    ptr: *mut T,
    initialized: bool
}

impl<T> const Default for SyncPointer<T> {
    fn default() -> Self {
        SyncPointer { ptr: null_mut(), initialized: false }
    }
}

impl<T> SyncPointer<T> {
    pub const fn new(ptr: *mut T) -> SyncPointer<T> {
        SyncPointer { ptr, initialized: true }
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }

    pub const fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl<T> AsMut<T> for SyncPointer<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}

unsafe impl<T: Send> Send for SyncPointer<T> {}

unsafe impl<T: Sync> Sync for SyncPointer<T> {}
