use std::mem::take;
use std::time::Instant;
use crate::leblanc::core::leblanc_handle::LeblancHandle;

use crate::leblanc::core::leblanc_object::{Callable, Reflect, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::utils::call_global_by_name;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;
use crate::unsafe_vec;

static mut GLOBALS: UnsafeVec<Method> = unsafe_vec![];
pub static mut HANDLES: UnsafeVec<LeblancHandle> = unsafe_vec![];

pub struct LeBlancRunner {
    globals: UnsafeVec<Method>,
}

impl LeBlancRunner {
    pub fn new(globals: UnsafeVec<Method>) -> LeBlancRunner {
        LeBlancRunner {
            globals,
        }
    }

    pub fn run_main(&mut self) {
        unsafe { GLOBALS = UnsafeVec::from(self.globals.clone()); }
        //let main_object = self.globals.iter_mut().filter(|g| g.typing == LeBlancType::Function).find(|g| g.reflect().downcast_ref::<Box<Method>>().unwrap().context.name == "main");

        let main_elapsed = Instant::now();
        let f = call_global_by_name("main", unsafe_vec![]).unwrap();
        if f.typing == 16 {
            let borrowed = f;
            let error: &LeblancError = borrowed.data.ref_data().unwrap();
            error.print_stack_trace();
        }
        println!("Running4");
        //println!("Final: {:#?}", f.lock().data);
        println!("Execution Elapsed: {}", main_elapsed.elapsed().as_secs_f64());
    }
}

#[inline(always)]
pub unsafe fn get_globals() -> &'static mut UnsafeVec<Method> {
    &mut GLOBALS
}

#[inline(always)]
pub fn get_handles() -> &'static mut UnsafeVec<LeblancHandle> {
    unsafe { &mut HANDLES }
}

