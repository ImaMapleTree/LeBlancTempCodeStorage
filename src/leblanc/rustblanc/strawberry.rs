
use parking_lot::lock_api::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
use parking_lot::{Mutex, RawMutex, RawRwLock, RwLock};

#[derive(Debug)]
pub struct Strawberry<T> {
    mutex: RwLock<T>,
}

impl<T> Strawberry<T> {
    pub fn new(data: T) -> Strawberry<T> {
        let mutex = RwLock::new(data);
        Strawberry {
            mutex,
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, RawRwLock, T> {
        self.mutex.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, RawRwLock, T> {
        self.mutex.write()
    }

    pub fn locked(&self) -> bool {
        self.mutex.is_locked()
    }

    //noinspection RsExternalLinter
    pub fn underlying_pointer(&self) -> &mut T {
        unsafe {&mut *self.mutex.data_ptr()}
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, RawRwLock, T>> {
        self.mutex.try_read()
    }
}

impl<T: Default + Clone> Strawberry<T> {
    pub fn force_unwrap(&self) -> T {
        let cloned = match unsafe {self.mutex.data_ptr().as_ref()} {
            Some(r) => r.clone(),
            None => T::default(),
        };
        cloned
    }
}

unsafe impl<T> Sync for Strawberry<T> {

}

unsafe impl<T> Send for Strawberry<T> {

}

impl<T: Default> Default for Strawberry<T> {
    fn default() -> Self {
        Strawberry::new(T::default())
    }
}