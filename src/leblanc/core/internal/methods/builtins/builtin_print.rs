use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::LeBlancType;

fn _BUILTIN_PRINT_(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    println!("{}", args[0].call("toString", &mut []).lock().unwrap().data.to_string());
    return Arc::new(Mutex::new(LeBlancObject::null()));
}

pub fn _BUILTIN_PRINT_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "call".to_string(),
            vec![LeBlancArgument::default(LeBlancType::Flex, 0)]
        ),
        _BUILTIN_PRINT_,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_PRINT_OBJECT_() -> LeBlancObject {
    return internal_method(_BUILTIN_PRINT_METHOD_());
}