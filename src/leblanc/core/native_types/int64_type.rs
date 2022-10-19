use std::collections::{BTreeSet};
use std::sync::{Arc};

use alloc::rc::Rc;
use std::cell::RefCell;
use fxhash::FxHashMap;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::leblanc::core::internal::methods::internal_math::_internal_inplace_add_;
use crate::leblanc::core::interpreter::HEAP;
use crate::leblanc::core::leblanc_argument::number_argset;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

lazy_static! {
    static ref INT_BASE: LBObject = base_int();
}

pub fn base_int() -> LBObject {
    let mut base_methods = Arc::unwrap_or_clone(base_methods());
    base_methods.insert(inplace_addition());

    LeBlancObject::new(
        LeBlancObjectData::Int64(0),
        LeBlancType::Int64,
        Arc::new(base_methods),
        FxHashMap::default(),
        VariableContext::empty(),
    )
}

pub fn leblanc_object_int64(integer: i64) -> LBObject {
    LeBlancObject::new(
        LeBlancObjectData::Int64(integer),
        LeBlancType::Int64,
        INT_BASE.methods.clone(),
        FxHashMap::default(),
        VariableContext::empty(),
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