use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_default_data::unsafe_empty_members;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_int(integer: i32) -> LeBlancObject {
    let base_methods = base_methods();


    LeBlancObject::new(
        LeBlancObjectData::Int(integer),
        LeBlancType::Int,
        base_methods,
        unsafe_empty_members(),
        VariableContext::empty(),
    )
}

/*fn int_addition_method() -> Method {
    let method_store = MethodStore {
        name: "addition".to_string(),
        arguments: number_argset(),
    };
    let mut method_tag = BTreeSet::new();
    method_tag.insert(MethodTag::Addition);

    return Method::new(
        method_store,
        _internal_add_number_,
        method_tag
    )
}*/

impl ToLeblanc for i32 {
    fn create(&self) -> LeBlancObject { leblanc_object_int(*self) }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { Rc::new(RefCell::new(self.create())) }
}

impl RustDataCast<i32> for LeBlancObjectData {
    fn clone_data(&self) -> Option<i32> {
        match self {
            LeBlancObjectData::Int(int) => Some(*int),
            _ => None,
        }
    }

    fn ref_data(&self) -> Option<&i32> {
        match self {
            LeBlancObjectData::Int(int) => Some(int),
            _ => None,
        }
    }

    fn mut_data(&mut self) -> Option<&mut i32> {
        match self {
            LeBlancObjectData::Int(int) => Some(int),
            _ => None,
        }
    }
}