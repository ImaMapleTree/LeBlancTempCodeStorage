use core::slice::{Iter, IterMut};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ComponentMap<T: PartialEq> {
    map: Vec<T>,
}

impl<T: PartialEq> ComponentMap<T> {
    pub fn new() -> ComponentMap<T> {
        ComponentMap {
            map: Vec::<T>::new()
        }
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.map.get(id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.map.get_mut(id)
    }

    pub fn put(&mut self, obj: T) -> usize {
        match self.map.iter().position(|cmpr| cmpr == &obj) {
            Some(id) => id,
            None => {
                let id = self.map.len();
                self.map.push(obj); id
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.map.iter_mut()
    }
}

impl<T: PartialEq> Default for ComponentMap<T> {
    fn default() -> Self {
        ComponentMap::new()
    }
}