use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::precompiled::Precompiled;
use crate::leblanc::core::interpreter::instruction_execution::execute_instruction;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::rustblanc::utils::Timing;

#[derive(Clone, Debug)]
pub struct LeblancHandle {
    pub constants: Vec<Arc<LeBlancObject>>,
    pub variable_context: HashMap<String, VariableContext>,
    pub variables: Vec<Arc<Mutex<LeBlancObject>>>,
    pub globals: Box<Vec<Arc<Mutex<LeBlancObject>>>>,
    pub instructions: Arc<Vec<Instruction>>,
    pub precompiled: Vec<Precompiled>,
    pub null: bool,
}

impl LeblancHandle {
    pub fn null() -> LeblancHandle {
        return LeblancHandle {
            constants: vec![],
            variable_context: HashMap::new(),
            variables: vec![],
            globals: Box::new(vec![]),
            instructions: Arc::new(vec![]),
            precompiled: vec![],
            null: true
        }
    }

    pub fn from_function_bytecode(mut bytecode: FunctionBytecode) -> LeblancHandle {
        let mut instructs: Vec<Instruction> = vec![];
        bytecode.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));
        let instructs = Arc::new(instructs);
        let constants: Vec<Arc<LeBlancObject>> = bytecode.constants().into_iter().map(|constant| Arc::new(constant.to_leblanc_object())).collect::<Vec<Arc<LeBlancObject>>>();
        let variable_context = bytecode.variables();


        return LeblancHandle {
            constants,
            variable_context,
            variables: vec![],
            globals: Box::new(vec![]),
            instructions: instructs,
            precompiled: vec![],
            null: false
        }
    }


    pub fn execute(&mut self, inputs: Vec<Arc<Mutex<LeBlancObject>>>) -> Arc<Mutex<LeBlancObject>> {
        println!("Executing new handle");
        let timings: HashMap<String, Timing> = HashMap::new();
        //unsafe { setup_timings(); }
        self.variables = inputs;
        let mut stack = vec![];
        let instruction_iter = self.instructions.clone();
        for instruction in instruction_iter.iter() {
            println!("Instruction: {:?}", instruction);
            if instruction.instruct == InstructionBase::Return {
                return stack.pop().unwrap();
            }
            let internal_handle = execute_instruction(instruction.base());
            internal_handle(self, instruction, &mut stack).unwrap();
            /*let duration = now.elapsed().as_secs_f64();
            let mut timing = *timings.get(&instruction.instruct.to_string()).unwrap_or(&Timing{count: 0, time: 0.0});
            timing.count += 1; timing.time += duration;
            timings.insert(instruction.instruct.to_string(), timing);*/
        }
        /*unsafe { print_timings(); }
        unsafe { leblanc::core::internal::methods::builtins::builtin_print::print_timings(); }
        println!("Execution: {:#?}",timings);*/
        return Arc::new(Mutex::new(LeBlancObject::null()));

    }
}



