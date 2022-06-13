use std::collections::HashMap;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_int64(integer: i64) -> LeBlancObject {
    let mut base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::Int64(integer),
        LeBlancType::Int64,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for i64 {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_int64(*self);
    }
}