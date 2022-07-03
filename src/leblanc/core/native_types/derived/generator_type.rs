use alloc::rc::Rc;

use core::fmt::{Display, Formatter};

use std::cell::RefCell;
use std::sync::Arc;
use fxhash::{FxHashMap};
use std::sync::Mutex;
use crate::leblanc::core::internal::transformed_iterator::TransformedIterator;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};

use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::core::native_types::derived::iterator_type::{iterator_methods, LeblancIterable, LeblancIterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::LeBlancType;

#[derive(Clone, Debug, PartialEq)]
pub struct LeblancGenerator {
    leblanc_handle: LeblancHandle
}

pub fn leblanc_object_generator(leblanc_handle: LeblancHandle) -> LeBlancObject {
    let base_methods = iterator_methods();

    let generator = LeblancGenerator {leblanc_handle};


    LeBlancObject::new(
        LeBlancObjectData::Iterator(LeblancIterator::new(Box::new(generator))),
        LeBlancType::Derived(DerivedType::Iterator),
        base_methods,
        Arc::new(Mutex::new(FxHashMap::default())),
        VariableContext::empty(),
    )
}

impl Display for LeblancGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "<Generator of {}>", self.leblanc_handle.name)
    }
}

impl LeblancIterable for LeblancGenerator {
    fn lb_next(&mut self) -> Arc<Mutex<LeBlancObject>> {
        self.leblanc_handle.execute_from_last_point().to_mutex()
    }
    fn has_next(&self) -> bool {
        self.leblanc_handle.current_instruct < self.leblanc_handle.instructions.len() as u64
    }

    fn reverse(&mut self) {
        todo!()
    }

    fn to_list(&mut self) -> LeblancList {
        let mut vec = vec![];
        while self.has_next() {
            vec.push(self.lb_next());
        }
        LeblancList::new(vec, )
    }

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=Arc<Mutex<LeBlancObject>>>> {
        Box::new(self.clone())
    }

    fn transformed(&mut self) -> Option<&mut TransformedIterator> { None }
}

impl Iterator for LeblancGenerator {
    type Item = Arc<Mutex<LeBlancObject>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.has_next() {
            true => Some(self.leblanc_handle.execute_from_last_point().to_mutex()),
            false => None
        }
    }
}
