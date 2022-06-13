use std::collections::HashMap;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_arch(arch: isize) -> LeBlancObject {
    let mut base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::Arch(arch),
        LeBlancType::Arch,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for isize {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_arch(*self);
    }
}