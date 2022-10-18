use fxhash::{FxHashMap};
use std::sync::Arc;
use crate::leblanc::rustblanc::strawberry::Strawberry;


use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

pub fn leblanc_object_short(integer: i16) -> LeBlancObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Short(integer),
        LeBlancType::Short,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for i16 {
    fn create(&self) -> LeBlancObject {
        leblanc_object_short(*self)
    }
    fn create_mutex(&self) -> LBObject { LBObject::from(self.create()) }
}