use std::sync::{Arc};
use fxhash::{FxHashMap, FxHashSet};
use lazy_static::lazy_static;
use parking_lot::Mutex;


use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

lazy_static! {
    static ref INT_BASE: LBObject = base_int();
}



pub fn base_int() -> LBObject {

    LeBlancObject::new(
        LeBlancObjectData::Int64(0),
        4,
        UnsafeVec::default(),
    )
}

#[inline(always)]
pub fn leblanc_object_int(integer: i32) -> LBObject {
    LeBlancObject::new(
        LeBlancObjectData::Int(integer),
        3,
        UnsafeVec::default()
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
    fn create(&self) -> LeBlancObject { leblanc_object_int(*self)._clone() }
    fn create_mutex(&self) -> LBObject { leblanc_object_int(*self) }
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