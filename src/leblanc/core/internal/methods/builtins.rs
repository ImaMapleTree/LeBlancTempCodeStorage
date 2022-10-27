








use crate::{lazystore, unsafe_vec};
use crate::leblanc::core::internal::methods::builtins::builtin_print::{_BUILTIN_PRINT_METHOD_, _BUILTIN_PRINT_OBJECT_};

use crate::leblanc::compiler::generator::generator_types::FunctionSignature;
use crate::leblanc::core::internal::methods::builtins::builtin_debug::{_BUILTIN_DEBUG_METHOD_, _BUILTIN_DEBUG_OBJECT_};
use crate::leblanc::core::internal::methods::builtins::builtin_debug::builtin_disassemble::{_BUILTIN_DISASSEMBLE_METHOD_, _BUILTIN_DISASSEMBLE_OBJECT_};
use crate::leblanc::core::internal::methods::builtins::builtin_type::{_BUILTIN_TYPE_METHOD_, _BUILTIN_TYPE_OBJECT_};
use crate::leblanc::core::method::Method;


use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::lazy_store::LazyStore;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub mod builtin_print;
pub mod builtin_debug;
pub mod builtin_type;

pub static BUILTIN_METHODS: i8 = 4;

pub fn create_lazy_functions() -> LazyStore<FunctionSignature> {
    lazystore![
        FunctionSignature::from_method(_BUILTIN_PRINT_METHOD_(), vec![LeBlancType::Null]),
        FunctionSignature::from_method(_BUILTIN_DISASSEMBLE_METHOD_(), vec![LeBlancType::Null]),
        FunctionSignature::from_method(_BUILTIN_DEBUG_METHOD_(), vec![LeBlancType::Null]),
        FunctionSignature::from_method(_BUILTIN_TYPE_METHOD_(), vec![LeBlancType::Null])
    ]
}

pub fn create_builtin_function_objects() -> Vec<LBObject> {
    vec![_BUILTIN_PRINT_OBJECT_(),
         _BUILTIN_DISASSEMBLE_OBJECT_(),
         _BUILTIN_DEBUG_OBJECT_(),
         _BUILTIN_TYPE_OBJECT_()]
}

pub fn create_builtin_function_methods() -> UnsafeVec<Method> {
    unsafe_vec![_BUILTIN_PRINT_METHOD_(),
         _BUILTIN_DISASSEMBLE_METHOD_(),
         _BUILTIN_DEBUG_METHOD_(),
         _BUILTIN_TYPE_METHOD_()]
}