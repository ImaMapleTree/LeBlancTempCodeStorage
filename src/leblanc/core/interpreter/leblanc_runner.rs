

use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::LeBlancType;

static mut GLOBALS: Vec<Arc<Strawberry<LeBlancObject>>> = vec![];


pub struct LeBlancRunner {
    globals: Vec<Arc<Strawberry<LeBlancObject>>>,
}

impl LeBlancRunner {
    pub fn new(globals: Vec<Arc<Strawberry<LeBlancObject>>>) -> LeBlancRunner {
        LeBlancRunner {
            globals,
        }
    }

    pub fn run_main(&mut self) {
        unsafe { GLOBALS = self.globals.to_vec(); }
        let main_object = self.globals.iter_mut().filter(|g| g.lock().typing == LeBlancType::Function).find(|g| g.reflect().downcast_ref::<Box<Method>>().unwrap().context.name == "main");

        let main_elapsed = Instant::now();
        let f = main_object.unwrap().call("main", &mut []).unwrap();
        if f.lock().typing == LeBlancType::Exception {
            let borrowed = f.lock();
            let error: &LeblancError = borrowed.data.ref_data().unwrap();
            error.print_stack_trace();
        }
        //println!("Final: {:#?}", f.lock().data);
        println!("Execution Elapsed: {}", main_elapsed.elapsed().as_secs_f64());
    }
}

pub unsafe fn get_globals() -> &'static Vec<Arc<Strawberry<LeBlancObject>>> {
    &GLOBALS
}

