use std::collections::BTreeSet;
use std::io;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::utils::Timings;
use crate::LeBlancType;

static mut TIMING: Timings = Timings { map: None };

static mut STDOUT: Option<io::Stdout> = None;

fn _BUILTIN_PRINT_(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let result = args[0].call_name("to_string").to_string() + "\n";
    unsafe {
        if STDOUT.is_none() {
            STDOUT = Some(io::stdout());
        }
        io::copy(&mut result.as_bytes(), &mut STDOUT.as_mut().unwrap()).unwrap();
    }
    return LeBlancObject::unsafe_null()
}

pub fn _BUILTIN_PRINT_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "print".to_string(),
            vec![LeBlancArgument::default(LeBlancType::Flex, 0)]
        ),
        _BUILTIN_PRINT_,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_PRINT_OBJECT_() -> LeBlancObject {
    return internal_method(_BUILTIN_PRINT_METHOD_());
}