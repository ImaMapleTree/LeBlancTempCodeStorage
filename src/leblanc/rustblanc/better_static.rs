use std::ops::{Deref, DerefMut};
use std::ptr::{null_mut, Unique};
use crate::leblanc::rustblanc::memory::pointer::{SyncPointer};


#[derive(Copy, Clone)]
pub struct BetterStatic<T> {
    init: fn() -> T,
    data: SyncPointer<T>,
    call: fn(&mut BetterStatic<T>) -> &mut T
}

impl<T> BetterStatic<T> {
    pub const fn new<>(f: fn() -> T) -> BetterStatic<T>
    {
        BetterStatic {
            init: f,
            data: SyncPointer::default(),
            call: BetterStatic::init_static
        }
    }

    #[inline(always)]
    fn init_static(bs: &mut BetterStatic<T>) -> &mut T {
        let item = (bs.init)();
        unsafe {
            bs.data = SyncPointer::new(Box::leak(Box::new(item)) as *mut T);
            bs.call = BetterStatic::_access;
            bs.data.as_mut()
        }
    }

    #[inline(always)]
    fn _access(bs: &mut BetterStatic<T>) -> &mut T {
        unsafe { bs.data.as_mut() }
    }

    #[inline(always)]
    pub fn access(&mut self) -> &mut T {
        let f = self.call;
        f(self)
    }

    #[inline(always)]
    pub fn unsafe_access(&mut self) -> &mut T {
        unsafe { self.data.as_mut() }
    }

    #[inline(always)]
    pub fn try_access(&self) -> Option<&T> {
        if self.data.is_initialized() {
            Some(unsafe { & *self.data.as_ptr()})
        } else {
            None
        }
    }

}

impl<T> Deref for BetterStatic<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.try_access().expect("Cannot deref. Static variable not initialized.")
    }
}

impl<T> DerefMut for BetterStatic<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.access()
    }
}

impl<T: PartialEq> PartialEq for BetterStatic<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        (*self.deref()).eq(other.deref())
    }
}