use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::compiler::compile_types::partial_function::PartialFunction;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::lib::random::random_core_module;

pub mod leblanc_colored;
pub mod datetime;
pub mod random;


pub fn get_core_modules() -> Vec<CoreModule> {
    vec![
        random_core_module()
    ]
}