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
    let mut bytecode = FunctionBytecode::new();
    let mut instruction_line = InstructionBytecode::new();
    instruction_line.set_line_number(0);
    instruction_line.add_instruction(21.to_hex(2), 1.to_hex(2)); //binary add = 2
    instruction_line.add_instruction(21.to_hex(2), 0.to_hex(2));
    instruction_line.add_instruction(2.to_hex(2), 0.to_hex(2));
    bytecode.add_instruction_line(instruction_line.generate());

    let a = LeblancHandle::from_function_bytecode(bytecode);

    let store = MethodStore::new("test".to_string(), LeBlancArgument::from_positional(&vec![LeBlancType::Flex, LeBlancType::Flex]));


    let method = Method::of_leblanc_handle(store, a, BTreeSet::new());
    let lb_obj = internal_method(method);
    let mut lbo = Arc::new(Mutex::new(lb_obj));

    let number1 = Arc::new(Mutex::new(leblanc_object_int64(100)));
    let number2 = Arc::new(Mutex::new(leblanc_object_int64(3)));

    lbo.call("test", &mut [number1.clone(), number2.clone()]);

    println!("Number 1: {:?}", number1.lock().unwrap().data);
    println!("Number 2: {:?}", number2.lock().unwrap().data);

    println!("---------------\n");

    let mut print = Arc::new(Mutex::new(_BUILTIN_PRINT_OBJECT_()));

    let result = print.call("call", &mut [number1.clone()]);
    println!("cloning print: {:?}", print.clone());
    println!("{:#?}", result);
    //print.call("call", &mut [result.clone()]);

    // raise error;
    println!("\n---------------");
    return LeBlancObject::null();

}







pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}