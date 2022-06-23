
use crate::leblanc::rustblanc::strawberry::{Either, Strawberry};
use std::time::Instant;
use crate::leblanc::core::exception::StackTrace;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect};
use crate::leblanc::core::method::Method;
use crate::LeBlancType;

static mut GLOBALS: Vec<Strawberry<LeBlancObject>> = vec![];

pub struct LeBlancRunner {
    globals: Vec<Strawberry<LeBlancObject>>,
    stack_trace: Vec<StackTrace>
}

impl LeBlancRunner {
    pub fn new(globals: Vec<Strawberry<LeBlancObject>>) -> LeBlancRunner {
        return LeBlancRunner {
            globals,
            stack_trace: vec![]
        }
    }

    pub fn run_main(&mut self) {
        unsafe { GLOBALS = self.globals.iter().cloned().collect(); }
        println!("Rung main");
        println!("Globals: {:?}", self.globals);
        let main_object = self.globals.iter_mut().filter(|g| g.loan().inquire_uncloned().either().typing == LeBlancType::Function)
            .filter(|g| g.reflect().downcast_ref::<Method>().unwrap().context.name == "main").next();

        let main_elapsed = Instant::now();
        main_object.unwrap().call("main", &mut []);
        println!("Execution Elapsed: {}", main_elapsed.elapsed().as_secs_f64());
    }
}

pub unsafe fn get_globals() -> &'static Vec<Strawberry<LeBlancObject>> {
    return &GLOBALS;
}