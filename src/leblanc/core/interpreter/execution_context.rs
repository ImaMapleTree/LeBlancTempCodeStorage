use std::mem::take;
use std::sync::Arc;
use crate::leblanc::core::interpreter::instruction_execution2::execute_instruction2;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::{get_globals, get_handles};
use crate::leblanc::core::leblanc_handle::{ExecutionSignal, LeblancHandle};
use crate::leblanc::core::leblanc_object::LeBlancObject;

use crate::leblanc::rustblanc::types::{LBObject, LeBlancStack};

pub struct ExecutionContext {
    pub handle_ref: &'static mut LeblancHandle,
    pub instruction_pointer: usize,
    pub instruct_total: usize,
    pub variables: Vec<LBObject>,
    pub stack: LeBlancStack
}

impl ExecutionContext {
    pub fn new(handle_index: usize, instruction_total: usize, inputs: Vec<LBObject>) -> Self {
        //let mut variables = vec![LeBlancObject::unsafe_null_quick(); get_handles().get_mut(handle_index).unwrap().variables.len()];
        /*inputs.iter_mut().enumerate().for_each(|(i, item)| {
            variables[i] = take(item)
        });*/
        let variables = inputs;

        ExecutionContext {
            handle_ref: get_handles().get_mut(handle_index).unwrap(),
            instruction_pointer: 0,
            instruct_total: instruction_total,
            variables,
            stack: LeBlancStack::new()
        }
    }

    pub fn get_constant(&self, id: usize) -> Option<&LBObject> {
        self.handle_ref.constants.get(id)
    }

    pub fn execute(&mut self) -> LBObject {
        while self.instruction_pointer < self.instruct_total {
            let instruct = self.handle_ref.instructions[self.instruction_pointer];
            self.instruction_pointer += 1;

            let iexec = execute_instruction2(instruct)(self, instruct);
            //self.debug(instruction);
            if let Err(error) = iexec {
                if let Instruction2::RETURN(..) = instruct {
                    return error;
                }
                return self.cascade_error(error)
            }
        }
        self.stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)
    }

    fn cascade_error(&self, err: LBObject) -> LBObject {
        println!("Errored: {:#?}", err);
        err
    }
}