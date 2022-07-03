use alloc::rc::Rc;
use core::fmt::Debug;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::internal::transformed_iterator::IterMutation::{Filter, Map};
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};

use crate::leblanc::core::native_types::derived::iterator_type::{LeblancIterable};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;

#[derive(Debug, Clone)]
pub struct TransformedIterator {
    inner_iterator: Box<dyn LeblancIterable>,
    transformations: Vec<IterMutation>
}

#[derive(Debug, Clone, PartialEq)]
pub enum IterMutation {
    Filter(LeblancHandle),
    Map(LeblancHandle)
}

impl TransformedIterator {
    pub fn new(inner_iterator: Box<dyn LeblancIterable>) -> TransformedIterator {
        TransformedIterator {
            inner_iterator,
            transformations: vec![]
        }
    }

    pub fn filter(&mut self, handle: LeblancHandle) {
        self.transformations.push(Filter(handle))
    }

    pub fn map(&mut self, handle: LeblancHandle) {
        self.transformations.push(Map(handle))
    }
}

impl LeblancIterable for TransformedIterator {
    fn lb_next(&mut self) -> Arc<Mutex<LeBlancObject>> {
        self.inner_iterator.lb_next()
    }

    fn has_next(&self) -> bool {
        self.inner_iterator.has_next()
    }

    fn reverse(&mut self) {
        self.inner_iterator.reverse()
    }

    fn to_list(&mut self) -> LeblancList {
        let mut iterator = self.inner_iterator.to_rust_iter();

        for transformation in &mut self.transformations {
            match transformation {
                Filter(handle) => {
                    iterator = Box::new(iterator.filter(|i| *handle.execute_lambda(&mut [i.clone()]).lock().unwrap().data.ref_data().unwrap()));
                }
                Map(handle) => {
                    iterator = Box::new(iterator.map(|i| handle.execute_lambda(&mut [i])));
                }
            }
        }

        LeblancList::new(iterator.collect::<Vec<Arc<Mutex<LeBlancObject>>>>())
    }

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=Arc<Mutex<LeBlancObject>>>> {
        todo!()
    }

    fn transformed(&mut self) -> Option<&mut TransformedIterator> {
        Some(self)
    }
}