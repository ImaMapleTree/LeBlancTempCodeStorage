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

pub mod leblanc;
pub mod playground;

extern crate alloc;
extern crate core;