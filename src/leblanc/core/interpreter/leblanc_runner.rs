

use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::leblanc::core::leblanc_handle::LeblancHandle;

use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

static mut GLOBALS: Vec<LBObject> = vec![];
static mut HANDLES: Vec<LeblancHandle> = vec![];

pub struct LeBlancRunner {
    globals: Vec<LBObject>,
}

impl LeBlancRunner {
    pub fn new(globals: Vec<LBObject>) -> LeBlancRunner {
        LeBlancRunner {
            globals,
        }
    }

    pub fn run_main(&mut self) {
        println!("Running");
        unsafe { GLOBALS = self.globals.to_vec(); }
        println!("Running2");
        let main_object = self.globals.iter_mut().filter(|g| g.typing == LeBlancType::Function).find(|g| g.reflect().downcast_ref::<Box<Method>>().unwrap().context.name == "main");
        println!("Running3");

        let main_elapsed = Instant::now();
        let f = main_object.unwrap().call("main", vec![]).unwrap();
        if f.typing == LeBlancType::Exception {
            let borrowed = f;
            let error: &LeblancError = borrowed.data.ref_data().unwrap();
            error.print_stack_trace();
        }
        println!("Running4");
        //println!("Final: {:#?}", f.lock().data);
        println!("Execution Elapsed: {}", main_elapsed.elapsed().as_secs_f64());
    }
}

pub unsafe fn get_globals() -> &'static mut Vec<LBObject> {
    &mut GLOBALS
}

pub fn get_handles() -> &'static mut Vec<LeblancHandle> {
    unsafe { &mut HANDLES }
}

