use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_boolean(boolean: bool) -> LeBlancObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Boolean(boolean),
        LeBlancType::Boolean,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for bool {
    fn create(&self) -> LeBlancObject {
        leblanc_object_boolean(*self)
    }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { Rc::new(RefCell::new(self.create())) }
}

impl RustDataCast<bool> for LeBlancObjectData {
    fn clone_data(&self) -> Option<bool> {
        match self {
            LeBlancObjectData::Boolean(bool) => Some(*bool),
            _ => None,
        }
    }

    fn ref_data(&self) -> Option<&bool> {
        match self {
            LeBlancObjectData::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    fn mut_data(&mut self) -> Option<&mut bool> {
        match self {
            LeBlancObjectData::Boolean(bool) => Some(bool),
            _ => None,
        }
    }
}