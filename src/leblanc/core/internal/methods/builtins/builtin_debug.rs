use alloc::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeSet;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};


use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{LeBlancObject};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

pub mod builtin_disassemble;

fn _BUILTIN_DEBUG_(_self: Arc<Strawberry<LeBlancObject>>, mut args: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let arg_length = args.len();
    println!("--------------------------------------------------------");
    println!("----------------------DEBUGGING-------------------------");
    println!("--------------------------------------------------------");
    for i in 0..arg_length {
        let _sep = if i == arg_length-1 { "\n" } else { " " };
        let arg = &mut args[i];
        dbg!(arg);
    }
    println!("--------------------------------------------------------");
    println!("--------------------------------------------------------");



    //io::copy(&mut result.as_bytes(), &mut STDOUT.as_mut().unwrap()).unwrap();
    LeBlancObject::unsafe_null()
}

pub fn _BUILTIN_DEBUG_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "debug".to_string(),
            vec![LeBlancArgument::variable(LeBlancType::Flex, 0)]
        ),
        _BUILTIN_DEBUG_,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_DEBUG_OBJECT_() -> LeBlancObject {
    internal_method(_BUILTIN_DEBUG_METHOD_())
}