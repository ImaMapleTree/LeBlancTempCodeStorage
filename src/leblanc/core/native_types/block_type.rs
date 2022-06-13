use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Copy, Debug)]
pub struct NativeBlock {

}

pub fn leblanc_object_block(block: NativeBlock) -> LeBlancObject {
    let mut base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::Block(block),
        LeBlancType::Block,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for NativeBlock {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_block(*self);
    }
}

impl Display for NativeBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}