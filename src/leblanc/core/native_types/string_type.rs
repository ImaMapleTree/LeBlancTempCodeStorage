use std::collections::{HashMap, HashSet};
use std::sync::{Arc};
use crate::leblanc::rustblanc::strawberry::Strawberry;

use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_, _internal_to_string_};
use crate::leblanc::core::internal::methods::internal_string::_internal_add_string;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::base_type::{base_clone_method, base_equals_method, base_expose_method, base_field_method, base_to_string_method, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::LeBlancType::Flex;

pub fn leblanc_object_string(string: String) -> LeBlancObject {
    let mut hash_set = HashSet::new();
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_field_method(), _internal_field_));
    hash_set.insert( string_addition_method());


    return LeBlancObject::new(
        LeBlancObjectData::String(string),
        LeBlancType::String,
        Arc::new(hash_set),
        HashMap::new(),
        VariableContext::empty(),
    )
}

impl ToLeblanc for String {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_string(self.clone());
    }
    fn create_mutex(&self) -> Strawberry<LeBlancObject> { return Strawberry::new(self.create()) }
}

pub fn string_addition_method() -> Method {
    let method_store = MethodStore::new("_ADD_".to_string(), LeBlancArgument::from_positional(&[Flex]));
    return Method::new(
        method_store,
        _internal_add_string,
        MethodTag::Addition.singleton()
    )
}