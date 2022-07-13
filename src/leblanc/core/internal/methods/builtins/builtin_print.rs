use std::collections::BTreeSet;
use std::io;


use alloc::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;

use crate::leblanc::core::native_types::LeBlancType;

fn _BUILTIN_PRINT_(_self: Arc<Strawberry<LeBlancObject>>, args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    let arg_length = args.len();
    for i in 0..arg_length {
        let sep = if i == arg_length-1 { "\n" } else { " " };
        let arg = &mut args[i];
        let result = match arg.call_name("to_string") {
            Ok(r) => r.to_string() + sep,
            Err(err) => return err
        };
        write!(io::stdout().lock(), "{}", result).unwrap();
        //io::stdout().write(result.as_bytes()).unwrap();
    }



        //io::copy(&mut result.as_bytes(), &mut STDOUT.as_mut().unwrap()).unwrap();
    LeBlancObject::unsafe_null()
}

pub fn _BUILTIN_PRINT_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "print".to_string(),
            vec![LeBlancArgument::variable(LeBlancType::Flex, 0)]
        ),
        _BUILTIN_PRINT_,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_PRINT_OBJECT_() -> LeBlancObject {
    internal_method(_BUILTIN_PRINT_METHOD_())
}