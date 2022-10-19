use std::mem::take;
use std::sync::Arc;
use crate::leblanc::core::interpreter::instruction_execution2::execute_instruction2;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::{get_globals, get_handles};
use crate::leblanc::core::leblanc_handle::{ExecutionSignal, LeblancHandle};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::include::lib::leblanc_colored::{Color, colorize};
use crate::leblanc::rustblanc::blueberry::{Blueberry, BlueberryVec, Quantum};

use crate::leblanc::rustblanc::types::{LBObject, LeBlancStack};

static DEBUG: bool = true;

pub struct ExecutionContext {
    pub handle_ref: &'static mut LeblancHandle,
    pub instruction_pointer: usize,
    pub instruct_total: usize,
    pub variables: Vec<LBObject>,
}

impl ExecutionContext {
    pub fn new(handle_index: usize, instruction_total: usize, context_length: usize, mut inputs: Vec<LBObject>) -> Self {
        //println!("Inputs: {:?}", inputs);
        //println!("Inputs: {:?}", inputs);
        let variables = inputs;
        //println!("Variables: {:?}", variables);

        //let variables = BlueberryVec::from(inputs.iter().map(|o| o.clone()).collect::<Vec<LeBlancObject>>());

        ExecutionContext {
            handle_ref: get_handles().get_mut(handle_index).unwrap(),
            instruction_pointer: 0,
            instruct_total: instruction_total,
            variables,
        }
    }

    #[inline]
    pub fn get_constant(&mut self, id: usize) -> Option<&LBObject> {
        self.handle_ref.constants.get(id)
    }

    #[inline]
    pub fn execute(&mut self) -> LBObject {
        while self.instruction_pointer < self.instruct_total {
            let instruct = self.handle_ref.instructions[self.instruction_pointer];
            self.instruction_pointer += 1;

            //self.debug(instruct);
            let iexec = execute_instruction2(instruct)(self, instruct);
            if let Err(error) = iexec {
                if let Instruction2::RETURN(..) = instruct {
                    return self.handle_ref.stack.pop().unwrap();
                }
                return self.cascade_error(error)
            }
        }
        self.handle_ref.stack.pop().unwrap()
    }

    fn cascade_error(&self, err: LBObject) -> LBObject {
        println!("Errored: {:#?}", err);
        err
    }

    fn debug(&self, instruction: Instruction2) {
        if DEBUG {println!("{} Normal Instruction: {:?}", colorize(self.handle_ref.name.to_string(), Color::Blue), instruction);}
    }
}