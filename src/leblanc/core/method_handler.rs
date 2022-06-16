pub mod internal_base_toString;
pub mod leblanc_handle;

use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::env::Args;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ptr::hash;
use crate::leblanc::core::method_handler::internal_base_toString::INTERNAL_TO_STRING;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::rustblanc::generic_data::GenericData;

pub enum MethodHandle {
    InternalToStringMethod,
    InternalEqualsMethod,
    InternalExposeMethod,
    InternalCloneMethod,
    InternalMethod(fn(&LeBlancObject, &[LeBlancObject]) -> LeBlancObject)
}

impl MethodHandle {
    pub fn run(&self, _self: &LeBlancObject, arguments: &[LeBlancObject]) -> LeBlancObject {
        return match self {
            MethodHandle::InternalMethod(method) => method(_self, arguments),
            _ => LeBlancObject::null()
        }
    }
}


impl Hash for MethodHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MethodHandle::InternalToStringMethod => "0".hash(state),
            MethodHandle::InternalEqualsMethod => "1".hash(state),
            MethodHandle::InternalExposeMethod => "2".hash(state),
            MethodHandle::InternalCloneMethod => "3".hash(state),
            MethodHandle::InternalMethod(func) => "4".hash(state)
        }
    }
}

impl PartialEq for MethodHandle {
    fn eq(&self, other: &Self) -> bool {
        let mut hasher = DefaultHasher::new();
        return self.hash(&mut hasher) == other.hash(&mut hasher);
    }
}

impl Eq for MethodHandle {}

impl Debug for MethodHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", dbg!(self))
    }
}