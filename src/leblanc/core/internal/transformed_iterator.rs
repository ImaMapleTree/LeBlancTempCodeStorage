use alloc::rc::Rc;
use core::fmt::Debug;
use std::cell::RefCell;
use crate::leblanc::core::internal::transformed_iterator::IterMutation::{Filter, Map};
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::derived::iterator_type::{LeblancIterable};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;

#[derive(Debug, Clone)]
pub struct TransformedIterator {
    inner_iterator: Box<dyn LeblancIterable>,
    transformations: Vec<IterMutation>
}

#[derive(Debug, Clone, PartialEq)]
pub enum IterMutation {
    Filter(Method),
    Map(Method)
}

impl TransformedIterator {
    pub fn new(inner_iterator: Box<dyn LeblancIterable>) -> TransformedIterator {
        TransformedIterator {
            inner_iterator,
            transformations: vec![]
        }
    }

    pub fn filter(&mut self, method: Method) {
        self.transformations.push(Filter(method))
    }

    pub fn map(&mut self, method: Method) {
        self.transformations.push(Map(method))
    }
}

impl LeblancIterable for TransformedIterator {
    fn lb_next(&mut self) -> Rc<RefCell<LeBlancObject>> {
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
                Filter(method) => {
                    iterator = Box::new(iterator.filter(|i| *method.run(LeBlancObject::unsafe_null(), &mut [i.clone()]).borrow().data.ref_data().unwrap()));
                }
                Map(method) => {
                    iterator = Box::new(iterator.map(|i| method.run(LeBlancObject::unsafe_null(), &mut [i])));
                }
            }
        }

        LeblancList::new(iterator.collect::<Vec<Rc<RefCell<LeBlancObject>>>>())
    }

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=Rc<RefCell<LeBlancObject>>>> {
        todo!()
    }

    fn transformed(&mut self) -> Option<&mut TransformedIterator> {
        return Some(self);
    }
}