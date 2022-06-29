use std::collections::BTreeSet;
use std::io;


use alloc::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::utils::Timings;
use crate::LeBlancType;

fn _BUILTIN_PRINT_(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    if args.len() == 0 {
        io::stdout().write("".as_bytes()).unwrap();
    }
    for arg in args {
        let result = match arg.call_name("to_string") {
            Ok(r) => r.to_string() + "\n",
            Err(err) => return err
        };
        io::stdout().write(result.as_bytes()).unwrap();
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