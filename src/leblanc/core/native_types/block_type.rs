use fxhash::{FxHashMap, FxHashSet};
use std::fmt::{Display, Formatter};

use crate::leblanc::rustblanc::strawberry::Strawberry;
use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct NativeBlock {

}

pub fn leblanc_object_block(block: NativeBlock) -> LeBlancObject {
    let base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::Block(block),
        LeBlancType::Block,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for NativeBlock {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_block(*self);
    }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { return Rc::new(RefCell::new(self.create())) }
}

impl Display for NativeBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}