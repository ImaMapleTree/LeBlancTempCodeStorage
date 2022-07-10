use std::fs;
use lalrpop_util::lalrpop_mod;
use crate::leblanc::compiler::grammar::error::report;
use crate::leblanc::compiler::grammar::token_analyzer::type_analysis;

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
pub mod grammar;

lalrpop_mod!(pub parser, "/leblanc/compiler/grammar/leblanc.rs");

pub fn compile(name: String) {
    let input = fs::read_to_string(name).unwrap();
    let result = parser::FileParser::new().parse(&input);
    match result {
        Ok(g) => type_analysis(g),
        Err(b) => report(b)
    }
}
