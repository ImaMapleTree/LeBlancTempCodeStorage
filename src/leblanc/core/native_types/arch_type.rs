use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

pub fn leblanc_object_arch(arch: isize) -> LeBlancObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Arch(arch),
        LeBlancType::Arch,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for isize {
    fn create(&self) -> LeBlancObject {
        leblanc_object_arch(*self)
    }
    fn create_mutex(&self) -> LBObject { LBObject::from(self.create()) }
}