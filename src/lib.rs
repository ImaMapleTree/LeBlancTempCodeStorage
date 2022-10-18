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
#![feature(let_else)]
#![feature(once_cell)]
#![feature(hash_set_entry)]
#![feature(try_trait_v2)]
#![feature(is_some_and)]
#![feature(path_file_prefix)]
#![feature(unsized_locals, unsized_fn_params)]
#![feature(pointer_byte_offsets)]
#![feature(sync_unsafe_cell)]
#![feature(ptr_internals)]

pub mod leblanc;
pub mod playground;

extern crate alloc;
extern crate core;