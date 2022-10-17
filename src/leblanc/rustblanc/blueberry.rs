use alloc::vec::Drain;
use core::fmt::Debug;
use std::marker::PhantomData;
use std::mem::take;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Blueberry<'a, T: ?Sized + 'a> {
    data: *mut T,
    phantom: PhantomData<&'a mut T>
}

impl<T> Clone for Blueberry<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Blueberry<'_, T> {}

impl<'a, T: ?Sized + 'a> Blueberry<'a, T> {
    pub fn new(data: &mut T) -> Blueberry<T> {
        Blueberry {
            data: data as *mut T,
            phantom: PhantomData
        }
    }
}

#[derive(Default, Debug)]
pub struct BlueberryVec<T> {
    inner: Vec<*mut T>,
    ownership: Vec<T>,
    lag: Vec<bool>
}

impl<T> BlueberryVec<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        BlueberryVec { inner: Vec::with_capacity(capacity), ownership: Vec::with_capacity(capacity), lag: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, item: T) {
        self.ownership.push(item);
        self.lag.push(true);
    }

    pub fn push_ref(&mut self, item: &mut T) {
        self.inner.push(item);
        self.lag.push(false);
    }

    pub fn get(&mut self, index: usize) -> Option<Blueberry<T>> {
        Some(Blueberry::new(unsafe { &mut **self.inner.get_mut(index)?}))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.ownership.pop()
    }

    pub fn pop_many(&mut self, n: usize) -> Drain<'_, T> {
        let length = self.ownership.len();
        self.ownership.drain(length-n..length)
    }
}

impl<T> From<Vec<T>> for BlueberryVec<T> {
    fn from(mut owned: Vec<T>) -> Self {
        let inner: Vec<*mut T> = owned.iter_mut().map(|mut o| o as *mut T).collect();
        let lag = vec![true; inner.len()];
        BlueberryVec { inner, ownership: owned, lag }
    }
}

impl<'a, T: ?Sized + 'a> Deref for Blueberry<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        unsafe { & *self.data }
    }
}

impl<'a, T: ?Sized + 'a> DerefMut for Blueberry<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

#[derive(Debug, Clone)]
pub struct BlueberryQuantum<'a, T> {
    owned: Option<T>,
    reference: Blueberry<'a, T>
}

impl<'a, T> BlueberryQuantum<'a, T> {
    pub fn owned(&mut self) -> Option<T> {
        take(&mut self.owned)
    }

    pub fn reference(&self) -> Blueberry<'a, T> {
        self.reference
    }
}