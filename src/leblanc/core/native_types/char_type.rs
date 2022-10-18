use fxhash::{FxHashMap, FxHashSet};
use std::sync::Arc;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

pub fn leblanc_object_char(ch: char) -> LBObject {
    let base_methods = Arc::new(FxHashSet::default());


    LeBlancObject::new(
        LeBlancObjectData::Char(ch),
        LeBlancType::Char,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}