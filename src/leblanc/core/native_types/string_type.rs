use std::collections::{HashMap, HashSet};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_string(string: String) -> LeBlancObject {
    let base_methods = HashSet::new();


    return LeBlancObject::new(
        LeBlancObjectData::String(string),
        LeBlancType::String,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}