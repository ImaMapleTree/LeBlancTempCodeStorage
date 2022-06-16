use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, Weak};
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::precompiled::Precompiled;
use crate::leblanc::core::interpreter::instruction_execution::execute_instruction;
use crate::leblanc::core::interpreter::instructions::Instruction;
use crate::leblanc::core::leblanc_object::LeBlancObject;

#[derive(Clone, Debug)]
pub struct LeblancHandle {
    pub constants: Vec<LeBlancObject>,
    pub variables: Vec<Arc<Mutex<LeBlancObject>>>,
    pub globals: Box<Vec<Arc<Mutex<LeBlancObject>>>>,
    pub instructions: Vec<Instruction>,
    pub precompiled: Vec<Precompiled>,
}

impl LeblancHandle {
    pub fn from_function_bytecode(mut bytecode: FunctionBytecode) -> LeblancHandle {
        let mut instructs: Vec<Instruction> = vec![];
        bytecode.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));

        return LeblancHandle {
            constants: Vec::new(),
            variables: Vec::new(),
            globals: Box::new(vec![]),
            instructions: instructs,
            precompiled: vec![]
        }
    }


    pub fn execute(&mut self, inputs: Vec<Arc<Mutex<LeBlancObject>>>) -> Arc<Mutex<LeBlancObject>> {
        println!("Executing!");
        self.variables = inputs;
        let mut stack = vec![];

        for instruction in self.instructions.clone().iter() {
            println!("Instruct: {:?}", instruction);
            let internal_handle = execute_instruction(instruction.base());
            internal_handle(self, instruction.arg, &mut stack).unwrap();

        }
        return Arc::new(Mutex::new(LeBlancObject::null()));

    }
}

