use fxhash::{FxHashMap, FxHashSet};
use std::sync::Arc;



use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::base_methods;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub fn leblanc_object_char(ch: char) -> LBObject {


    LeBlancObject::new(
        LeBlancObjectData::Char(ch),
        1,
        UnsafeVec::default()
    )
}