use core::fmt::{Debug, Formatter};
use core::slice::{Iter, IterMut};
use std::mem::{replace, take};


#[derive(Default)]
pub struct LazyStore<T: PartialEq + Lazy> {
    items: Vec<T>
}

impl<T: PartialEq + Lazy> LazyStore<T> {
    pub fn add(&mut self, item: T) -> usize {
        let size = self.items.len();
        self.items.push(item);
        size
    }

    pub fn get_or_add(&mut self, item: T, strategy: Strategy) -> (usize, bool) {
        if let Some(index) = self.index(&item, strategy) {
            (index, true)
        } else {
            (self.add(item), false)
        }
    }

    pub fn index(&self, item: &T, strategy: Strategy) -> Option<usize> {
        self.items.iter().position(|i| i.scan(item, strategy))
    }

    pub fn contains(&self, item: &T, strategy: Strategy) -> bool {
        self.index(item, strategy).is_some()
    }

    pub fn similar(&self, item: &T, strategy: Strategy) -> Option<&T> {
        self.items.get(self.index(item, strategy)?)
    }

    pub fn similar_mut(&mut self, item: &T, strategy: Strategy) -> Option<&mut T> {
        let index = self.index(item, strategy)?;
        self.items.get_mut(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    pub fn replace(&mut self, index: usize, new: T) -> T {
        replace(&mut self.items[index], new)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.items.iter_mut()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl <T: PartialEq + Lazy + Default> LazyStore<T> {
    pub fn take(&mut self, index: usize) -> T{
        take(&mut self.items[index])
    }
}

impl<T: PartialEq + Lazy + Debug> Debug for LazyStore<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LazyStore")
            .field("items", &self.items)
            .finish()
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub enum Strategy {
    LAZY,
    #[default]
    STANDARD,
    RUST
}

pub trait Lazy {
    fn scan(&self, other: &Self, strategy: Strategy) -> bool;
}

impl<T: Lazy + PartialEq + Default> From<Vec<T>> for LazyStore<T> {
    fn from(v: Vec<T>) -> Self {
        let mut s = LazyStore::default();
        v.into_iter().for_each(|item| {s.add(item);});
        s
    }
}