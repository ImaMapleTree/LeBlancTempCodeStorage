use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};

use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_, _internal_to_string_};
use crate::leblanc::core::internal::methods::internal_math::_internal_add_number_;
use crate::leblanc::core::leblanc_argument::{LeBlancArgument, number_argset};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::LeBlancType::*;
use crate::leblanc::rustblanc::types::LBObject;

static mut BASE_METHODS: Option<Arc<FxHashSet<Method>>> = None;

pub trait ToLeblanc {
    fn create(&self) -> LeBlancObject;
    fn create_mutex(&self) -> LBObject;
}



pub fn base_methods() -> Arc<FxHashSet<Method>> {
    unsafe {
        if BASE_METHODS.is_none() {
            let mut hash_set = FxHashSet::with_capacity_and_hasher(6, FxBuildHasher::default());
            hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
            hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
            hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
            hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
            hash_set.insert(Method::default(base_field_method(), _internal_field_));
            hash_set.insert( base_addition_method());
            BASE_METHODS = Some(Arc::new(hash_set));
        }
        return BASE_METHODS.as_ref().unwrap().clone();
    }
}

pub fn internal_method(method: Method) -> LeBlancObject {
    let mut methods = Arc::unwrap_or_clone( base_methods());
    methods.insert(method.clone());
    LeBlancObject {
        data: LeBlancObjectData::Function(Box::new(method)),
        typing: Function,
        methods: Arc::new(methods),
        members: FxHashMap::default(),
        context: VariableContext::empty()
    }
}

pub fn base_to_string_method() -> MethodStore {
    MethodStore::no_args("to_string".to_string())
}

pub fn base_expose_method() -> MethodStore {
    MethodStore::no_args("expose".to_string())
}

pub fn base_equals_method() -> MethodStore {
    MethodStore {
        name: "equals".to_string(),
        arguments: vec![LeBlancArgument::default(Flex, 0)],
    }
}

pub fn base_clone_method() -> MethodStore {
    MethodStore::no_args("clone".to_string())
}

pub fn base_field_method() -> MethodStore { MethodStore::new("field".to_string(),
                                                                vec![LeBlancArgument::default(LeBlancType::String, 0)])}


pub fn base_addition_method() -> Method {
    let method_store = MethodStore::new("_ADD_".to_string(), number_argset(0));
    Method::new(
        method_store,
        _internal_add_number_,
        MethodTag::Addition.singleton()
    )
}