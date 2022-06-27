use fxhash::{FxHashMap};
use std::fmt::{Display, Formatter};


use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct NativeBlock {

}

pub fn leblanc_object_block(block: NativeBlock) -> LeBlancObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Block(block),
        LeBlancType::Block,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for NativeBlock {
    fn create(&self) -> LeBlancObject {
        leblanc_object_block(*self)
    }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { Rc::new(RefCell::new(self.create())) }
}

impl Display for NativeBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}