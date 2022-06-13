use std::collections::{BTreeSet, HashMap, HashSet};
use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_};
use crate::leblanc::core::internal::methods::internal_math::_internal_add_number_;
use crate::leblanc::core::method_handler::MethodHandle;
use crate::leblanc::core::leblanc_argument::{LeBlancArgument, number_argset};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_handler::internal_base_toString::INTERNAL_TO_STRING;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::LeBlancType::*;

pub trait ToLeblanc {
    fn create(&self) -> LeBlancObject;
}

pub fn base_methods() -> HashSet<Method> {
    let mut method_map = HashSet::new();
    method_map.insert(Method::default(base_to_string_method(), INTERNAL_TO_STRING));
    method_map.insert(Method::default(base_expose_method(), _internal_expose_));
    method_map.insert(Method::default(base_equals_method(), INTERNAL_TO_STRING));
    method_map.insert(Method::default(base_clone_method(), INTERNAL_TO_STRING));
    method_map.insert(Method::default(base_field_method(), _internal_field_));
    return method_map;
}

pub fn base_to_string_method() -> MethodStore {
    return MethodStore::no_args("toString".to_string());
}

fn base_expose_method() -> MethodStore {
    return MethodStore::no_args("expose".to_string());
}

fn base_equals_method() -> MethodStore {
    return MethodStore {
        name: "equals".to_string(),
        arguments: vec![LeBlancArgument::default(LeBlancType::RealFlex, 0)],
    }
}

fn base_clone_method() -> MethodStore {
    return MethodStore::no_args("clone".to_string());
}

fn base_field_method() -> MethodStore { return MethodStore::new("field".to_string(),
                                                                vec![LeBlancArgument::default(LeBlancType::String, 0)])}


fn base_addition_method() -> Method {
    let method_store = MethodStore::new("add".to_string(), number_argset());
    return Method::new(
        method_store,
        _internal_add_number_,
        BTreeSet::new()
    )
}