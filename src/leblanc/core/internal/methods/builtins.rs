

use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::utils::{decode_hex, encode_hex};
use crate::lazystore;
use crate::leblanc::core::internal::methods::builtins::builtin_print::{_BUILTIN_PRINT_METHOD_, _BUILTIN_PRINT_OBJECT_};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::compiler::compile_types::partial_function::PartialFunction;
use crate::leblanc::compiler::generator::generator_types::FunctionSignature;
use crate::leblanc::core::internal::methods::builtins::builtin_debug::{_BUILTIN_DEBUG_METHOD_, _BUILTIN_DEBUG_OBJECT_};
use crate::leblanc::core::internal::methods::builtins::builtin_debug::builtin_disassemble::{_BUILTIN_DISASSEMBLE_METHOD_, _BUILTIN_DISASSEMBLE_OBJECT_};
use crate::leblanc::core::internal::methods::builtins::builtin_type::{_BUILTIN_TYPE_METHOD_, _BUILTIN_TYPE_OBJECT_};
use crate::leblanc::core::interpreter::leblanc_runner::get_handles;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::lazy_store::LazyStore;
use crate::leblanc::rustblanc::types::LBObject;

pub mod builtin_print;
pub mod builtin_debug;
pub mod builtin_type;

pub static BUILTIN_METHODS: i8 = 4;

pub fn create_partial_functions() -> Vec<PartialFunction> {
    vec![
        PartialFunction::from_method(_BUILTIN_PRINT_METHOD_(), vec![LeBlancType::Null]),
        PartialFunction::from_method(_BUILTIN_DISASSEMBLE_METHOD_(), vec![LeBlancType::Null]),
        PartialFunction::from_method(_BUILTIN_DEBUG_METHOD_(), vec![LeBlancType::Null]),
        PartialFunction::from_method(_BUILTIN_TYPE_METHOD_(), vec![LeBlancType::Null])
    ]
}

pub fn create_lazy_functions() -> LazyStore<FunctionSignature> {
    lazystore![
        FunctionSignature::from_method(_BUILTIN_PRINT_METHOD_(), vec![LeBlancType::Null]),
        FunctionSignature::from_method(_BUILTIN_DISASSEMBLE_METHOD_(), vec![LeBlancType::Null]),
        FunctionSignature::from_method(_BUILTIN_DEBUG_METHOD_(), vec![LeBlancType::Null]),
        FunctionSignature::from_method(_BUILTIN_TYPE_METHOD_(), vec![LeBlancType::Null])
    ]
}

pub fn create_builtin_function_objects() -> Vec<LBObject> {
    vec![_BUILTIN_PRINT_OBJECT_().to_mutex(),
         _BUILTIN_DISASSEMBLE_OBJECT_().to_mutex(),
         _BUILTIN_DEBUG_OBJECT_().to_mutex(),
         _BUILTIN_TYPE_OBJECT_().to_mutex()]
}