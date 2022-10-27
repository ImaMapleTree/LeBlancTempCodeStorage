

use crate::leblanc::core::interpreter::instruction_execution2::execute_instruction2;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::{HANDLES};
use crate::leblanc::core::leblanc_handle::{LeblancHandle};
use crate::leblanc::core::leblanc_object::LeBlancObject;

use crate::leblanc::include::lib::leblanc_colored::{Color, colorize};
use crate::leblanc::rustblanc::memory::heap::HeapRef;


use crate::leblanc::rustblanc::types::{LBObjArgs, LBObject};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

static DEBUG: bool = true;

pub struct ExecutionContext {
    pub handle_ref: &'static mut LeblancHandle,
    pub instruction_pointer: usize,
    pub instruct_total: usize,
    pub variables: UnsafeVec<LBObject>,
    pub should_return: bool,
}

impl ExecutionContext {
    pub fn new(handle_index: usize, instruction_total: usize, variables: LBObjArgs) -> Self {
        //println!("Inputs: {:?}", inputs);
        //println!("Inputs: {:?}", inputs);
        //println!("Variables: {:?}", variables);

        //let variables = BlueberryVec::from(inputs.iter().map(|o| o.clone()).collect::<Vec<LeBlancObject>>());

        ExecutionContext {
            handle_ref: unsafe {HANDLES.get_unchecked_mut(handle_index)},
            instruction_pointer: 0,
            instruct_total: instruction_total,
            variables,
            should_return: false,
        }
    }

    #[inline(always)]
    pub fn get_constant(&mut self, id: usize) -> LBObject {
        unsafe {self.handle_ref.constants.get_unchecked(id)}.clone()
    }

    #[inline(always)]
    pub fn execute(&mut self) -> LBObject {
        //println!("Executing {}", colorize(self.handle_ref.name.to_string(), Color::Blue));
        while !self.should_return && self.instruction_pointer < self.instruct_total {
            let instruct = *unsafe{ self.handle_ref.instructions.get_unsafe(self.instruction_pointer)};
            self.instruction_pointer += 1;
            //self.debug(instruct);

            execute_instruction2(instruct)(self, instruct);
        }
        if let Some(return_value) = self.handle_ref.stack.pop_quick() {
            return return_value;
        }
        LeBlancObject::unsafe_null()
    }

    fn cascade_error(&self, err: LBObject) -> LBObject {
        println!("Errored: {:#?}", err);
        err
    }

    fn debug(&self, instruction: Instruction2) {
        if DEBUG {println!("{} Normal Instruction: {:?}", colorize(self.handle_ref.name.to_string(), Color::Blue), instruction);}
    }
}