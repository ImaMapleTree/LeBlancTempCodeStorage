use std::sync::{Arc, Mutex};
use crate::leblanc::core::exception::StackTrace;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect};
use crate::leblanc::core::method::Method;
use crate::LeBlancType;

static mut GLOBALS: Vec<Arc<Mutex<LeBlancObject>>> = vec![];

pub struct LeBlancRunner {
    globals: Vec<Arc<Mutex<LeBlancObject>>>,
    stack_trace: Vec<StackTrace>
}

impl LeBlancRunner {
    pub fn new(globals: Vec<Arc<Mutex<LeBlancObject>>>) -> LeBlancRunner {
        return LeBlancRunner {
            globals,
            stack_trace: vec![]
        }
    }

    pub fn run_main(&mut self) {
        unsafe { GLOBALS = self.globals.iter().cloned().collect(); }

        let main_object = self.globals.iter_mut().filter(|g| g.lock().unwrap().typing == LeBlancType::Function)
            .filter(|g| g.reflect().downcast_ref::<Method>().unwrap().context.name == "main").next();
        main_object.unwrap().call("main", &mut []);
    }
}

pub unsafe fn get_globals() -> &'static Vec<Arc<Mutex<LeBlancObject>>> {
    return &GLOBALS;
}