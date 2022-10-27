#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(const_trait_impl)]
#![feature(unchecked_math)]
#![feature(new_uninit)]
#![feature(core_intrinsics)]
#![feature(try_reserve_kind)]

extern crate alloc;


use crate::lbvm::heap::Heap;
use crate::test::test_lbvm::setup;

mod lbvm;
pub mod leblanc;
pub mod builtin;
pub(crate) mod arch;
pub(crate) mod test;
pub mod leblanc_type;

#[cfg(any(target_env = "msvc", debug_assertions))]
use mimalloc::MiMalloc;

#[cfg(any(target_env = "msvc", debug_assertions))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


const MIN_BLOCK_SIZE: usize = TYPICAL_PAGE_SIZE * 1024;
const TYPICAL_PAGE_SIZE: usize = 4096;



fn main() {
    //let h = Heap::new(MIN_BLOCK_SIZE, TYPICAL_PAGE_SIZE);
    setup()
}
