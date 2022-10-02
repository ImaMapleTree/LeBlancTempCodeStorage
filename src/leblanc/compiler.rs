use alloc::rc::Rc;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use lalrpop_util::lalrpop_mod;
use sharedlib::{FuncTracked, FuncUnsafe, LibRc, LibUnsafe, Symbol};
use crate::leblanc::compiler::generator::generate;
use crate::leblanc::compiler::parser::ast::{Component, push_byte_location};
use crate::leblanc::compiler::parser::error::report;
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::bridge::{_unsafe_get_module_export, _unsafe_get_shared_object, _unsafe_set_module_export, _unsafe_set_shared_object, set_mod_swapper, set_obj_swapper};

use crate::leblanc::rustblanc::types::{BIModFunc, BIObjFunc, BModGetter, BModSwapper, BObjGetter, BObjSwapper};

pub mod import;
pub mod parser;
pub mod bytecode;
pub mod compile_types;
pub mod generator;

lalrpop_mod!(pub lalrpop, "/leblanc/compiler/parser/leblanc.rs");

pub fn compile(name: String) {
    let input = fs::read_to_string(name).unwrap();

    let bytes = input.clone().into_bytes();


    let mut line_number: usize = 1;
    let mut symbol_number: usize = 0;
    for byte in bytes {
        unsafe { push_byte_location((line_number, symbol_number)); }
        symbol_number += 1;
        if byte == 10 {
            line_number += 1;
            symbol_number = 0;
        }
    }


    let result = lalrpop::FileParser::new().parse(&input);
    match result {
        Ok(g) => generate(g),
        Err(b) => report(b)
    }
}
