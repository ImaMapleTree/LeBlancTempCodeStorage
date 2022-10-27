use std::collections::{BTreeSet};
use std::sync::{Arc};



use fxhash::{FxHashMap, FxHashSet};


use lazy_static::lazy_static;

use crate::leblanc::core::internal::methods::internal_math::_internal_inplace_add_;

use crate::leblanc::core::leblanc_argument::number_argset;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

/*lazy_static! {
    static ref INT_BASE: LBObject = base_int();
}*/

/*pub fn base_int() -> LBObject {
    let mut base_methods =  wild_heap().alloc_with(|| (*base_methods()).clone());
    base_methods.insert(inplace_addition());

    LeBlancObject::new(
        LeBlancObjectData::Int64(0),
        LeBlancType::Int64,
        UnsafeVec::default()
    )
}*/

pub fn leblanc_object_int64(integer: i64) -> LBObject {
    LeBlancObject::new(
        LeBlancObjectData::Int64(integer),
        4,
        UnsafeVec::default()
    )
}


impl ToLeblanc for i64 {
    fn create(&self) -> LeBlancObject {
        leblanc_object_int64(*self)._clone()
    }
    fn create_mutex(&self) -> LBObject { leblanc_object_int64(*self) }
}

fn inplace_addition() -> Method {
    let method_store = MethodStore::new("inplace_addition".to_string(), number_argset(0));
    let mut method_tag = BTreeSet::new();
    method_tag.insert(MethodTag::InPlaceAddition);
    Method::new(
        method_store,
        _internal_inplace_add_,
        method_tag
    )
}