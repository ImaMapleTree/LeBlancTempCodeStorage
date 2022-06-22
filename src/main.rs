#![feature(downcast_unchecked)]
#![feature(core_ffi_c)]
#![feature(dec2flt)]
#![feature(core_intrinsics)]
#![feature(test)]
#![feature(arc_unwrap_or_clone)]
#![feature(mutex_unlock)]
#![feature(get_mut_unchecked)]
#![feature(fn_traits)]
#![feature(total_cmp)]

extern crate core;
extern crate alloc;

use std::io;
use std::time::Instant;
use clicolors_control::set_colors_enabled;
use crate::leblanc::compiler::compile::compile;
use crate::leblanc::compiler::compile_error_reporter::error_report;
use crate::leblanc::compiler::compile_types::CompilationMode;
use crate::leblanc::compiler::compile_types::full_reader::read_file;
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{BraceOpen, Semicolon};
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab;
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab::BOUNDARY;
use crate::leblanc::compiler::fabric::Fabric;
use crate::leblanc::compiler::token_stack_generator::create_stack;
use crate::leblanc::compiler::tokenizer::create_tokens;
use crate::leblanc::core::interpreter::interactive::start;
use crate::leblanc::core::interpreter::run;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::relationship::to_node_vec;


pub mod leblanc;
pub mod playground;

static INTERACTIVE: bool = false;


fn main() -> io::Result<()> {
    let DEBUG = true;
    //playground::playground();

    if INTERACTIVE {
        start();
    }
    let now = Instant::now();




    //exit(0);






    set_colors_enabled(true);

    compile("test.lb".to_string(), CompilationMode::Full);


    let bc = read_file("test.lb".to_string());
    run(bc);

    let elapsed = now.elapsed();
    println!("Total Elapsed: {}", elapsed.as_secs_f64());

    Ok(())
}

#[bench]
pub fn test_main() -> std::io::Result<()> {
    main()
}