#![feature(downcast_unchecked)]
#![feature(core_ffi_c)]
#![feature(dec2flt)]
#![feature(core_intrinsics)]
#![feature(test)]
#![feature(arc_unwrap_or_clone)]
#![feature(mutex_unlock)]
#![feature(get_mut_unchecked)]
#![feature(fn_traits)]
#![feature(ptr_const_cast)]
#![feature(cell_leak)]
#![feature(ptr_as_uninit)]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//#![allow(clippy::all)]

// Pedantic
#![deny(clippy::borrow_as_ptr, clippy::cloned_instead_of_copied, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::copy_iterator,
clippy::filter_map_next, clippy::map_unwrap_or, )]

#![deny(clippy::box_collection, clippy::boxed_local, clippy::cmp_owned, clippy::expect_fun_call, clippy::extend_with_drain,
clippy::format_in_format_args, clippy::format_push_string, clippy::iter_nth, clippy::iter_overeager_cloned, clippy::large_const_arrays,
clippy::large_enum_variant, clippy::manual_memcpy, clippy::manual_str_repeat, clippy::map_entry, clippy::missing_spin_loop,
clippy::needless_collect, clippy::or_fun_call, clippy::redundant_allocation, clippy::redundant_clone, clippy::single_char_pattern,
clippy::slow_vector_initialization, clippy::to_string_in_format_args, clippy::unnecessary_to_owned, clippy::useless_vec, clippy::vec_init_then_push
)]

extern crate alloc;
extern crate core;
#[macro_use] extern crate lalrpop_util;
extern crate core;
extern crate core;
//

use std::io;

use std::time::Instant;
use clicolors_control::set_colors_enabled;
use crate::leblanc::compiler::compile::compile;

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

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


fn main() -> io::Result<()> {
    let _DEBUG = true;
    //playground::playground();

    if INTERACTIVE {
        start();
    }
    let now = Instant::now();




    //exit(0);






    set_colors_enabled(true);
    lex("test.lb".to_string());

   /* //compile("test.lb".to_string(), CompilationMode::Full);


    let bc = read_file("test.lb".to_string());
    run(bc);

    let elapsed = now.elapsed();
    println!("Total Elapsed: {}", elapsed.as_secs_f64());*/

    Ok(())
}