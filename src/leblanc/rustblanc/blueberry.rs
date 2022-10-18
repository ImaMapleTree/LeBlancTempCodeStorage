use alloc::vec::Drain;
use core::fmt::Debug;
use core::slice::{Iter, IterMut};
use std::iter::Map;
use std::marker::PhantomData;
use std::mem;
use std::mem::take;
use std::ops::{Deref, DerefMut};
use std::ptr::{addr_of_mut, null_mut};



struct Null {}

static mut NP: *mut Null = null_mut();

#[derive(Debug)]
pub struct Blueberry<'a, T: ?Sized + 'a> {
    pointer: *mut T,
    phantom: PhantomData<&'a mut T>
}

impl<T> Clone for Blueberry<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Blueberry<'_, T> {}

impl<'a, T: ?Sized + 'a> Blueberry<'_, T> {
    #[inline]
    pub fn new(data: &mut T) -> Blueberry<T> {
        Blueberry {
            pointer: data as *mut T,
            phantom: PhantomData
        }
    }

    #[inline]
    pub fn immutable(data: &T) -> Blueberry<T> {
        Blueberry {
            pointer: data as *const T as *mut T,
            phantom: PhantomData
        }
    }

    #[inline]
    pub fn pointer(&self) -> *mut T {
        self.pointer
    }

    #[inline]
    pub unsafe fn null<R>() -> Blueberry<'a, R> {
        Blueberry::from(NP as *mut R)
    }
}

impl<'a, T> From<*mut T> for Blueberry<'_, T> {
    #[inline]
    fn from(data: *mut T) -> Self {
        Blueberry {
            pointer: data,
            phantom: PhantomData
        }
    }
}

#[derive(Debug)]
pub struct BlueberryVec<T> {
    owned: Vec<(Option<T>, *mut T)>,
}

pub trait BlueberryPush<T> {
    fn push(&mut self, item: T);
}

impl<T: Debug> BlueberryVec<T> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        BlueberryVec { owned: Vec::with_capacity(capacity) }
    }

    #[inline]
    pub fn get(&mut self, index: usize) -> Option<Blueberry<T>> {
        Some(Blueberry::from(self.owned[index].1))
    }

    #[inline]
    pub fn set(&mut self, index: usize, item: T) -> Result<(), ()> {
        self.owned[index] = (Some(item), unsafe { NP as *mut T});
        if let Some(inner) = &mut unsafe { self.owned.get_unchecked_mut(index)}.0 {
            self.owned[index].1 = inner as *mut T;
        }
        Err(())
    }

    #[inline]
    pub fn set_ref(&mut self, index: usize, item: Blueberry<T>) {
        self.owned[index] = (None, item.pointer);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Quantum<T>> {
        //println!("POPPING Owned: {:?}", self.owned);
        let (obj, pointer) = self.owned.pop()?;
        Some(Quantum { owned: obj, reference: Blueberry::from(pointer) })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.owned.len()
    }

    #[inline]
    pub unsafe fn clone(&self) -> BlueberryVec<T> {
        let owned: Vec<(Option<T>, *mut T)> = self.owned.iter()
            .map(|(obj, pointer)| (None, *pointer))
            .collect();
        BlueberryVec { owned }
    }
}

impl<T: Clone + Debug> BlueberryVec<T> {
    #[inline]
    pub fn set_smart(&mut self, index: usize, item: Quantum<T>) -> Result<(), ()> {
        if item.is_owned() {
            self.set(index, item.to_owned())
        } else {
            self.set_ref(index, item.reference);
            Ok(())
        }
    }
}

impl<T> Default for BlueberryVec<T> {
    fn default() -> Self {
        BlueberryVec { owned: Vec::new() }
    }
}

impl<T: Debug> BlueberryPush<T> for BlueberryVec<T> {
    #[inline]
    fn push(&mut self, item: T) {
        //println!("Pushed: {:?}", item);
        let index = self.owned.len();
        self.owned.push((Some(item), unsafe { NP as *mut T}));
        if let Some(inner) = &mut unsafe { self.owned.get_unchecked_mut(index)}.0 {
            self.owned[index].1 = inner as *mut T;
        }
        //println!("Current: {:?}", self.owned[index]);
    }
}

impl<T> BlueberryPush<&mut T> for BlueberryVec<T> {
    #[inline]
    fn push(&mut self, item: &mut T) {
        self.owned.push((None, item));
    }
}

impl<T> BlueberryPush<Blueberry<'_, T>> for BlueberryVec<T> {
    #[inline]
    fn push(&mut self, item: Blueberry<'_, T>) {
        self.owned.push((None, item.pointer));
    }
}

impl<T> BlueberryPush<*mut T> for BlueberryVec<T> {
    #[inline]
    fn push(&mut self, item: *mut T) {
        self.owned.push((None, item))
    }
}

impl<T> From<Vec<T>> for BlueberryVec<T> {
    #[inline]
    fn from(owned: Vec<T>) -> Self {
        let owned: Vec<(Option<T>, *mut T)> = owned.into_iter()
            .map(|item| (Some(item), null_mut()))
            .collect();
        let mut blueberry = BlueberryVec { owned };
        blueberry.owned.iter_mut().for_each(|t| {
            if let Some(inner) = &mut t.0 {
                t.1 = inner as *mut T;
            }
        });
        blueberry
    }
}

impl<T: Clone> From<Vec<Quantum<T>>> for BlueberryVec<T> {
    #[inline]
    fn from(quantums: Vec<Quantum<T>>) -> Self {
        let owned: Vec<(Option<T>, *mut T)> = quantums.into_iter()
            .map(|item| (item.owned, item.reference.pointer))
            .collect();
        let mut blueberry = BlueberryVec { owned };
        blueberry.owned.iter_mut().for_each(|t| {
            if t.0.is_some() {
                if let Some(inner) = &mut t.0 {
                    t.1 = inner as *mut T;
                }
            }
        });
        blueberry
    }
}

impl<'a, T: ?Sized + 'a> Deref for Blueberry<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        unsafe { & *self.pointer }
    }
}

impl<'a, T: ?Sized + 'a> DerefMut for Blueberry<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.pointer }
    }
}

#[derive(Debug, Clone)]
pub struct Quantum<T: 'static> {
    owned: Option<T>,
    reference: Blueberry<'static, T>,
}

impl<T> Default for Quantum<T> {
    fn default() -> Self {
        Quantum { owned: None, reference: unsafe { Blueberry::<T>::null() } }
    }
}

impl<T> Quantum<T> {
    #[inline]
    pub fn new(item: T) -> Quantum<T> {
        Quantum { owned: Some(item), reference: unsafe { Blueberry::<T>::null() } }
    }

    #[inline]
    pub fn owned(&mut self) -> Option<T> {
        take(&mut self.owned)
    }

    #[inline]
    pub fn reference(&self) -> Blueberry<'_, T> {
        if let Some(inner) = &self.owned {
            Blueberry::immutable(inner)
        } else { self.reference }

    }

    #[inline]
    pub fn is_owned(&self) -> bool {
        self.owned.is_some()
    }
}

impl<'a, T: Clone> Quantum<T> {
    #[inline]
    pub fn to_owned(self) -> T {
        if let Some(inner) = self.owned {
            inner
        } else { (*self.reference).clone() }
    }
}

unsafe impl<T: Send> Send for Quantum<T> {}