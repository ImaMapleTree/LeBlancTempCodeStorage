use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_char(ch: char) -> LeBlancObject {
    let base_methods = Arc::new(HashSet::new());


    return LeBlancObject::new(
        LeBlancObjectData::Char(ch),
        LeBlancType::Char,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}