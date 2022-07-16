use alloc::rc::Rc;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use lalrpop_util::lalrpop_mod;
use sharedlib::{FuncTracked, FuncUnsafe, LibRc, LibUnsafe, Symbol};
use crate::leblanc::compiler::parser::ast::{Component, push_byte_location};
use crate::leblanc::compiler::parser::error::report;
use parser::rules::declaration_rule::declaration_analysis;
use crate::leblanc::compiler::parser::function_sorter::sort_functions;
use crate::leblanc::compiler::parser::generator::generate_bytecode;
use crate::leblanc::compiler::parser::import_manager::{CompiledImport, import_dynamic, scan_imports};
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::bridge::{_unsafe_get_module_export, _unsafe_get_shared_object, _unsafe_set_module_export, _unsafe_set_shared_object, set_mod_swapper, set_obj_swapper};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::rustblanc::types::{BIModFunc, BIObjFunc, BModGetter, BModSwapper, BObjGetter, BObjSwapper};

pub mod import;
pub mod parser;
pub mod bytecode;
pub mod compile_types;

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
    println!("Current dir: {:?}", std::env::current_dir().unwrap().join(path.clone()));
    println!("Import Path: {:?}", path);
    let path = std::env::current_dir().unwrap().join(path);

    return if path.extension().unwrap().eq("dll")  || path.extension().unwrap().eq("so") {
        let core_mod = import_dynamic(path);
        let compiled_import = CompiledImport { name, components: vec![], module: Some(core_mod) };
        vec![compiled_import]
    } else {
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
        let self_compiled = CompiledImport { name, components: unwrapped, module: None };
        let imports = scan_imports(self_compiled);
        imports
    }
}

fn _compile(mut tokens: Vec<Component>) {
    let self_compiled = CompiledImport { name: String::from("_MAIN_"), components: tokens, module: None };
    let mut modules = scan_imports(self_compiled);
    let mut type_map = declaration_analysis(&mut modules);

    sort_functions(&mut type_map);

    println!("TYPE MAP: {:#?}", type_map);
    generate_bytecode(modules, type_map);
}

