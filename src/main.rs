#![feature(downcast_unchecked)]
#![feature(core_ffi_c)]
#![feature(dec2flt)]
#![feature(core_intrinsics)]

extern crate core;

use std::io;
use std::fs::File;
use std::process::exit;
use std::time::Instant;
use clicolors_control::set_colors_enabled;
use crate::leblanc::compiler::compile::compile;
use crate::leblanc::compiler::compile_error_reporter::error_report;
use crate::leblanc::compiler::compile_types::CompilationMode;
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{BraceOpen, BracketOpen, Semicolon};
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab;
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab::BOUNDARY;
use crate::leblanc::compiler::partial::PartialFabric;
use crate::leblanc::compiler::token_stack_generator::create_stack;
use crate::leblanc::compiler::tokenizer::create_tokens;
use crate::leblanc::core::interpreter::interactive::start;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::lib::leblanc_colored::ColorString;
use crate::leblanc::rustblanc::relationship::to_node_vec;


pub mod leblanc;
pub mod playground;

static INTERACTIVE: bool = false;


fn main() -> io::Result<()> {
    let DEBUG = true;
    playground::playground();

    if INTERACTIVE {
        start();
    }




    //exit(0);
    let now = Instant::now();





    set_colors_enabled(true);

    compile("test.lb".to_string(), CompilationMode::Full);


    let elapsed = now.elapsed();
    println!("Elapsed: {:.6?}", elapsed);

    Ok(())
}