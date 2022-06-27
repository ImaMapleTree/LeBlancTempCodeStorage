use fxhash::{FxHashMap, FxHashSet};

use crate::leblanc::rustblanc::strawberry::Strawberry;
use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_short(integer: i16) -> LeBlancObject {
    let base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::Short(integer),
        LeBlancType::Short,
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for i16 {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_short(*self);
    }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { return Rc::new(RefCell::new(self.create())) }
}