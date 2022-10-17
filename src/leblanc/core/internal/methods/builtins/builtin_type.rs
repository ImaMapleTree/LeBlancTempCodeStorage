use alloc::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeSet;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::{internal_method, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

fn _BUILTIN_TYPE_(_self: Arc<Strawberry<LeBlancObject>>, args: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    args[0].read().typing.to_string().create_mutex()
}

pub fn _BUILTIN_TYPE_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "type".to_string(),
            vec![LeBlancArgument::variable(LeBlancType::Flex, 0)]
        ),
        _BUILTIN_TYPE_,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_TYPE_OBJECT_() -> LeBlancObject {
    internal_method(_BUILTIN_TYPE_METHOD_())
}