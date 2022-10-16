use std::mem::take;
use std::sync::Arc;
use crate::leblanc::core::interpreter::instruction_execution2::execute_instruction2;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_handle::{ExecutionSignal, LeblancHandle};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::rustblanc::types::{LBObject, LeBlancStack};

pub struct ExecutionContext {
    pub handle_ref: &'static mut Arc<Strawberry<LeblancHandle>>,
    pub instruction_pointer: usize,
    pub instruct_total: usize,
    pub variables: Vec<LBObject>,
    pub stack: LeBlancStack
}

impl ExecutionContext {
    pub fn new(handle_index: usize, inputs: &mut [LBObject]) -> Self {
        let pointer = &mut unsafe {get_globals()}.get_mut(handle_index).unwrap().underlying_pointer().data.get_mut_inner_method().unwrap().leblanc_handle.underlying_pointer();
        let mut variables = vec![LeBlancObject::unsafe_null(); pointer.variables.len()];
        inputs.iter_mut().enumerate().for_each(|(i, item)| {
            variables[i] = take(item)
        });
        ExecutionContext {
            handle_ref: &mut unsafe {get_globals()}.get_mut(handle_index).unwrap().underlying_pointer().data.get_mut_inner_method().unwrap().leblanc_handle,
            instruction_pointer: 0,
            instruct_total: 0,
            variables,
            stack: LeBlancStack::new()
        }
    }

    pub fn get_constant(&self, id: usize) -> Option<&LBObject> {
        return self.handle_ref.underlying_pointer().constants.get(id);
    }

    pub fn execute(&mut self) -> LBObject {
        while self.instruction_pointer < self.instruct_total {
            let instruct = self.next();
            if instruct.get_inum() == 21 {
                let ret = self.stack.pop().unwrap();
                //println!("Returning: {:?}", ret);
                return ret;
            }
            let iexec = execute_instruction2(instruct)(self, instruct);
            //self.debug(instruction);
            if let Err(error) = iexec {
                return self.cascade_error(error)
            }
        }
        self.stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)
    }

    pub fn next(&mut self) -> Instruction2 {
        let i = self.handle_ref.underlying_pointer().instructions[self.instruction_pointer];
        self.instruction_pointer += 1; i
    }

    fn cascade_error(&self, err: LBObject) -> LBObject {
        println!("Errored: {:#?}", err);
        err
    }
}