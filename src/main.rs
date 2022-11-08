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
#![feature(try_trait_v2)]
#![feature(path_file_prefix)]
#![feature(unsized_locals, unsized_fn_params)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(try_reserve_kind)]
#![feature(new_uninit)]
#![feature(unchecked_math)]
#![feature(const_trait_impl)]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(incomplete_features)]
#![feature(let_chains)]
#![feature(pointer_is_aligned)]
#![feature(layout_for_ptr)]
#![feature(ptr_to_from_bits)]
#![feature(ptr_metadata)]
#![feature(const_size_of_val_raw)]
#![feature(const_align_of_val_raw)]
#![feature(const_mut_refs)]
#![feature(arbitrary_enum_discriminant)]



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
//


use std::{io};
use std::mem::take;
use std::process::exit;


use std::time::Instant;
use clicolors_control::set_colors_enabled;



use crate::leblanc::compiler::file_system::read_file;

use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::core::interpreter::leblanc_runner::get_handles;
use crate::leblanc::core::interpreter::{run};
use crate::leblanc::core::leblanc_handle::LeblancHandle;


use crate::leblanc::rustblanc::path::ZCPath;


pub mod leblanc;
pub mod playground;

static INTERACTIVE: bool = false;



#[cfg(all(not(target_env = "msvc"), not(debug_assertions)))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(not(target_env = "msvc"), not(debug_assertions)))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[cfg(any(target_env = "msvc", debug_assertions))]
use mimalloc::MiMalloc;
use ::leblanc::leblanc::core::heap::HEAP;
use crate::leblanc::core::heap::heap;

#[cfg(any(target_env = "msvc", debug_assertions))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


fn main() -> io::Result<()> {
    get_handles().push(LeblancHandle::null());
    let _DEBUG = true;
    let now = Instant::now();

    //playground::playground();
    //exit(0);

    set_colors_enabled(true);
    let mut generator = CodeGenerator::default();

    generator.compile(ZCPath::new("test/test.lb"));
    if generator.reporter.has_errors() {
        generator.reporter.report();
    }



    /*let bc = read_file("test/test.lb".to_string());
    run(bc);*/

    let elapsed = now.elapsed();
    println!("Total Elapsed: {}", elapsed.as_secs_f64());

    //drop(take(heap().access()));
    Ok(())
}

/*fn call_dynamic() -> Result<LeBlancObject, Box<dyn std::error::Error>> {
    unsafe {
        let path = Path::new("random.dll");
        println!("Path: {:#?}", path.is_file());
        println!("Path: {:#?}", path);
        let lib = libloading::Library::new(path).unwrap();
        let method : libloading::Symbol<unsafe fn() -> *mut CoreModule> = lib.get(b"MODULE").unwrap();
        let mut module = Box::from_raw(method());
        let mut m = module.exp_methods.get_mut(0).unwrap().clone();
        println!("M: {:#?}", m);

        let lbo = m.method.run(LeBlancObject::unsafe_null(), &mut [leblanc_object_string(String::from("Hello World!"))]).force_unwrap().clone();

        println!("I am okay");
        Ok(lbo)
    }
}*/

