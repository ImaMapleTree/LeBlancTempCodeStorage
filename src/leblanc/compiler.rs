use std::fs;
use std::path::PathBuf;
use lalrpop_util::lalrpop_mod;
use crate::leblanc::compiler::parser::ast::{Component, push_byte_location};
use crate::leblanc::compiler::parser::error::report;
use parser::rules::declaration_rule::declaration_analysis;
use crate::leblanc::compiler::parser::generator::generate_bytecode;
use crate::leblanc::compiler::parser::import_manager::{CompiledImport, scan_imports};

pub mod char_reader;
pub mod compile;
pub mod driver;
pub mod tokenizer;
pub mod symbols;
pub mod lang;
pub mod compiler_util;
pub mod compile_error_reporter;
pub mod fabric;
pub mod syntax_rules;
pub mod token_stack_generator;
pub mod identifier;
pub mod module_resolver;
pub mod compile_types;
pub mod import;
pub mod parser;
pub mod bytecode;

lalrpop_mod!(pub lalrpop, "/leblanc/compiler/parser/leblanc.rs");

pub fn compile2(name: String) {
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
        Ok(g) => _compile(g),
        Err(b) => report(b)
    }
}

pub fn compile_import(name: String, path: PathBuf) -> Vec<CompiledImport> {
    println!("Import Path: {:?}", path);
    let input = fs::read_to_string(path).unwrap();

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
    let mut unwrapped = result.unwrap();
    let self_compiled = CompiledImport { name, components: unwrapped };
    let imports = scan_imports(self_compiled);
    return imports;
}

fn _compile(mut tokens: Vec<Component>) {
    let self_compiled = CompiledImport { name: String::from("_MAIN_"), components: tokens };
    let mut modules = scan_imports(self_compiled);
    let type_map = declaration_analysis(&mut modules);


    println!("TYPE MAP: {:#?}", type_map);
    generate_bytecode(modules, type_map);
}
