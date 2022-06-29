

use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::utils::{decode_hex, encode_hex};
use strum::{EnumIter, IntoEnumIterator};
use strum_macros::EnumVariantNames;
use strum::VariantNames;
use crate::leblanc::core::internal::methods::builtins::builtin_print::{_BUILTIN_PRINT_METHOD_, _BUILTIN_PRINT_OBJECT_};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::compiler::compile_types::partial_function::PartialFunction;
use crate::leblanc::core::internal::methods::builtins::builtin_debug::builtin_disassemble::{_BUILTIN_DISASSEMBLE_METHOD_, _BUILTIN_DISASSEMBLE_OBJECT_};
use crate::LeBlancType;

pub mod builtin_print;
pub mod builtin_debug;

#[derive(Debug, PartialEq, Eq, EnumVariantNames, strum_macros::Display, EnumIter)]
pub enum BuiltinFunctions {
    Print,
    Disassemble
}

pub fn create_partial_functions() -> Vec<PartialFunction> {
    vec![PartialFunction::from_method(_BUILTIN_PRINT_METHOD_(), vec![LeBlancType::Null]), PartialFunction::from_method(_BUILTIN_DISASSEMBLE_METHOD_(), vec![LeBlancType::Null])]
}

pub fn create_builtin_function_objects() -> Vec<Rc<RefCell<LeBlancObject>>> {
    vec![_BUILTIN_PRINT_OBJECT_().to_mutex(), _BUILTIN_DISASSEMBLE_OBJECT_().to_mutex()]
}

impl Hexable for BuiltinFunctions {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let variants: &[&'static str] = BuiltinFunctions::VARIANTS;
        encode_hex(&(variants.iter().position(|s| *s.to_string() == self.to_string()).unwrap() as u32).to_be_bytes()[4-bytes..4])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let mut bytes = decode_hex(hex).unwrap();
        while bytes.len() < 4 { bytes.insert(0, 0) };
        let instruct_number = u32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap());
        BuiltinFunctions::iter().enumerate().find(|&(i, _)| i == instruct_number as usize).unwrap().1
    }
}