// 3 + 2


use std::collections::BTreeSet;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, passed_args_to_types};
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::double_type::leblanc_object_double;
use crate::leblanc::core::native_types::float_type::leblanc_object_float;
use crate::leblanc::core::native_types::int128_type::leblanc_object_int128;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use std::fmt::Write;
use std::num::ParseIntError;
use std::sync::{Arc, Mutex};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::core::bytecode::ToBytecode;
use crate::leblanc::core::internal::methods::builtins::builtin_print::_BUILTIN_PRINT_OBJECT_;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::core::native_types::int64_type::leblanc_object_int64;
use crate::leblanc::rustblanc::Hexable;

fn test() -> LeBlancObject {

    println!("\n---------------");
    return LeBlancObject::null();

}







pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}