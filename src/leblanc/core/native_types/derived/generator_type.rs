use core::fmt::{Display, Formatter};
use std::collections::HashMap;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};

use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::core::native_types::derived::iterator_type::{iterator_methods, LeblancIterable, LeblancIterator};
use crate::LeBlancType;

#[derive(Clone, Debug, PartialEq)]
pub struct LeblancGenerator {
    leblanc_handle: LeblancHandle
}

pub fn leblanc_object_generator(leblanc_handle: LeblancHandle) -> LeBlancObject {
    let base_methods = iterator_methods();

    let generator = LeblancGenerator {leblanc_handle};


    return LeBlancObject::new(
        LeBlancObjectData::Iterator(LeblancIterator::new(Box::new(generator))),
        LeBlancType::Derived(DerivedType::Iterator),
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}

impl Display for LeblancGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "<Generator of {}>", self.leblanc_handle.name)
    }
}

impl LeblancIterable for LeblancGenerator {
    fn next(&mut self) -> LeBlancObject {
        self.leblanc_handle.execute_from_last_point().borrow()._clone()
    }
    fn has_next(&self) -> bool {
        return self.leblanc_handle.current_instruct < self.leblanc_handle.instructions.len() as u64
    }
}


