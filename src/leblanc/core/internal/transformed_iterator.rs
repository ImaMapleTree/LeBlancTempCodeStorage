use core::fmt::Debug;


use crate::leblanc::core::internal::transformed_iterator::IterMutation::{Filter, Map};
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::{RustDataCast};

use crate::leblanc::core::native_types::derived::iterator_type::{LeblancIterable};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::unsafe_vec;

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
    fn lb_next(&mut self) -> LBObject {
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
                    iterator = Box::new(iterator.filter(|i| *handle.execute_lambda(unsafe_vec![i.clone()]).data.ref_data().unwrap()));
                }
                Map(handle) => {
                    iterator = Box::new(iterator.map(|i| handle.execute_lambda(unsafe_vec![i])));
                }
            }
        }

        LeblancList::new(iterator.collect::<Vec<LBObject>>())
    }

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=LBObject>> {
        todo!()
    }

    fn transformed(&mut self) -> Option<&mut TransformedIterator> {
        Some(self)
    }
}