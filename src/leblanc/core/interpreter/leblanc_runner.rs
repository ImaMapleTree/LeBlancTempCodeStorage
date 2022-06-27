

use alloc::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;
use crate::leblanc::core::exception::StackTrace;
use crate::leblanc::core::interpreter::instructions::Instruction;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect, RustDataCast, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::LeBlancType;

static mut GLOBALS: Vec<Rc<RefCell<LeBlancObject>>> = vec![];


pub struct LeBlancRunner {
    globals: Vec<Rc<RefCell<LeBlancObject>>>,
    stack_trace: Vec<StackTrace>
}

impl LeBlancRunner {
    pub fn new(globals: Vec<Rc<RefCell<LeBlancObject>>>) -> LeBlancRunner {
        LeBlancRunner {
            globals,
            stack_trace: vec![]
        }
    }

    pub fn run_main(&mut self) {
        unsafe { GLOBALS = self.globals.to_vec(); }
        let main_object = self.globals.iter_mut().filter(|g| g.borrow().typing == LeBlancType::Function).find(|g| g.reflect().downcast_ref::<Method>().unwrap().context.name == "main");

        let main_elapsed = Instant::now();
        let f = main_object.unwrap().call("main", &mut []);
        if f.borrow().typing == LeBlancType::Exception {
            let borrowed = f.borrow();
            let error: &LeblancError = borrowed.data.ref_data().unwrap();
            error.print_stack_trace();
        }
        println!("Final: {:#?}", f.borrow().data);
        println!("Execution Elapsed: {}", main_elapsed.elapsed().as_secs_f64());
    }
}

pub unsafe fn get_globals() -> &'static Vec<Rc<RefCell<LeBlancObject>>> {
    &GLOBALS
}

