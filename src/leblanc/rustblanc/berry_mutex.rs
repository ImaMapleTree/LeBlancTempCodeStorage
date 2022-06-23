use core::borrow::BorrowMut;
use core::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::ptr;
use std::ptr::null_mut;
use std::sync::{Arc, LockResult, Mutex, MutexGuard, TryLockError, TryLockResult};
use crate::leblanc::core::leblanc_handle::LeblancHandle;

/*static mut COUNT: u64 = 0;
static mut COUNT2: u64 = 0;
static mut COUNT3: u64 = 0;*/

#[derive(Debug)]
pub struct Berry<T: Clone + Debug + ?Sized> {
    inner_ptr: *mut T,
    berry_lock: BerryLock<T>,
    locked: bool
}

/*pub unsafe fn print_counts() {
    println!("C1: {}", COUNT);
    println!("C2: {}", COUNT2);
    println!("C3: {}", COUNT3);
}*/

impl<T: Clone + Debug + ?Sized> Berry<T> {
    pub fn new(mut t: T) -> Berry<T> {
        //println!("New");
        let mut lock = BerryLock { data: Arc::new(Mutex::new(t.clone())), parent_ptr: null_mut() };
        let mut berry = Berry { inner_ptr: null_mut(), berry_lock: lock, locked: true };
        //let mut berry = Berry { inner_ptr: null_mut(), berry_lock: lock, error_lock: lock2, locked: false };
        berry.inner_ptr = berry.berry_lock.data.lock().unwrap().deref_mut() as *mut T;
        //println!("uh oh");
        berry
    }



    pub fn acquire(&mut self) -> Result<&mut BerryLock<T>, BerryLock<*mut T>> {
        /*println!("Acquire");
        println!("Locked: {}", self.locked);*/
        //unsafe { println!("Locked: {:#?}", self.inner_ptr.read()); }
       // unsafe {COUNT3 += 1;}

        if self.locked {
            //unsafe {COUNT += 1;}
            let berry_lock = unsafe {BerryLock { data: Arc::new(Mutex::new(self.inner_ptr)), parent_ptr: null_mut() }};
            //println!("About to return");
            unsafe { return Err(berry_lock); }
        }

        //unsafe {COUNT2 += 1;}


        self.locked = true;
        self.berry_lock.parent_ptr = self as *mut Berry<T>;
        //println!("Berry lock: {:#?}", self.berry_lock);
        return Ok(&mut self.berry_lock);
    }

    /*pub fn lock(&mut self) -> MutexGuard<T> {
        println!("Locking");
        let mut guard = match self.arc.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        return guard;
    }*/
}

impl<T: Clone + Debug> Clone for Berry<T> {
    fn clone(&self) -> Self {
        match self.locked || self.berry_lock.is_locked() {
            true => unsafe{Berry::new((&*self.inner_ptr).clone())},
            false => unsafe {
                let mut lock = BerryLock { data: self.berry_lock.data.clone(), parent_ptr: null_mut() };
                let mut berry = Berry { inner_ptr: null_mut(), berry_lock: lock, locked: false };
                berry.inner_ptr = berry.berry_lock.data.lock().unwrap().deref_mut() as *mut T;
                berry

            }
        }
    }
}

#[derive(Debug)]
pub struct BerryLock<T: Clone + Debug> {
    data: Arc<Mutex<T>>,
    parent_ptr: *mut Berry<T>,
}

impl<T: Clone + Debug> BerryLock<T> {
    pub fn get(&mut self) -> Arc<Mutex<T>> {
        //println!("Getting lock");
        return self.data.clone();
    }

    pub fn is_locked(&self) -> bool { return self.data.try_lock().is_err() }
}

impl<T: Clone + Debug> Drop for BerryLock<T> {
    fn drop(&mut self) {
        //println!("Drop");
        if !self.parent_ptr.is_null() {
            //unsafe { println!("Parent: {:#?}", &*self.parent_ptr) }
            unsafe { (*self.parent_ptr).locked = false; }
        }
    }
}