use std::ptr::{addr_of, null, null_mut};
use parking_lot::lock_api::MutexGuard;
use parking_lot::{Mutex, RawMutex};

#[derive(Debug)]
pub struct Strawberry<T: Clone + Default> {
    mutex: Mutex<T>,
}

impl<T: Clone + Default> Strawberry<T> {
    pub fn new(data: T) -> Strawberry<T> {
        let mutex = Mutex::new(data);
        Strawberry {
            mutex,
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, RawMutex, T> {
        self.mutex.lock()
    }

    pub fn locked(&self) -> bool {
        self.mutex.is_locked()
    }

    pub fn force_unwrap(&self) -> T {
        let cloned = match unsafe {self.mutex.data_ptr().as_ref()} {
            Some(r) => r.clone(),
            None => T::default(),
        };
        cloned
    }

    pub fn underlying_pointer(&self) -> &mut T {
        unsafe {&mut *self.mutex.data_ptr()}
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, RawMutex, T>> {
        self.mutex.try_lock()
    }
}

/*pub struct StrawberryPointer<T> {
    pointer: *mut T
}

impl<T> StrawberryPointer<T> {
    pub fn unwrap(&self) -> &mut T {
    }

}*/

unsafe impl<T: Clone + Default> Sync for Strawberry<T> {

}

unsafe impl<T: Clone + Default> Send for Strawberry<T> {

}

impl<T: Clone + Default> Default for Strawberry<T> {
    fn default() -> Self {
        Strawberry::new(T::default())
    }
}