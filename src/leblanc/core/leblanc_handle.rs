use std::collections::HashMap;
use std::sync::{Arc};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::time::Instant;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::precompiled::Precompiled;
use crate::leblanc::core::interpreter::instruction_execution::execute_instruction;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::rustblanc::utils::{Timings};

static DEBUG: bool = false;
static TIME_DEBUG: bool = false;
static STACK_DEBUG: bool = false;

static mut TIMINGS: Timings = Timings { map: None};


#[derive(Debug)]
pub struct LeblancHandle {
    pub name: String,
    pub constants: Arc<Vec<Arc<LeBlancObject>>>,
    pub variable_context: Arc<HashMap<String, VariableContext>>,
    pub variables: Vec<Strawberry<LeBlancObject>>,
    pub globals: Box<Vec<Strawberry<LeBlancObject>>>,
    pub instructions: Arc<Vec<Instruction>>,
    pub precompiled: Vec<Precompiled>,
    pub current_instruct: u64,
    pub null: bool,
}

impl LeblancHandle {
    pub fn null() -> LeblancHandle {
        return LeblancHandle {
            name: "".to_string(),
            constants: Arc::new(vec![]),
            variable_context: Arc::new(HashMap::new()),
            variables: vec![],
            globals: Box::new(vec![]),
            instructions: Arc::new(vec![]),
            precompiled: vec![],
            current_instruct: 0,
            null: true
        }
    }

    pub fn from_function_bytecode(mut bytecode: FunctionBytecode) -> LeblancHandle {
        let mut instructs: Vec<Instruction> = vec![];
        bytecode.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));
        let instructs = Arc::new(instructs);
        let constants: Vec<Arc<LeBlancObject>> = bytecode.constants().into_iter().map(|constant| Arc::new(constant.to_leblanc_object())).collect::<Vec<Arc<LeBlancObject>>>();
        let variable_context = bytecode.variables();
        let name = bytecode.name();
        let context_length = variable_context.len();
        return LeblancHandle {
            name,
            constants: Arc::new(constants),
            variable_context: Arc::new(variable_context),
            variables: Vec::with_capacity(context_length),
            globals: Box::new(vec![]),
            instructions: instructs,
            precompiled: vec![],
            current_instruct: 0,
            null: false
        }
    }

    pub fn execute_no_args(&mut self) -> Strawberry<LeBlancObject> {
        unsafe { TIMINGS.setup() }
        self.current_instruct = 0;
        let mut stack = Vec::with_capacity(20);
        while self.current_instruct < self.instructions.len() as u64 {
            let instruction = self.instructions[self.current_instruct as usize];
            if DEBUG {println!("Normal Instruction: {:?}", instruction);}
            if instruction.instruct == InstructionBase::Return {
                return stack.pop().unwrap();
            }
            if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.base());
            let now = Instant::now();
            internal_handle(self, &instruction, &mut stack).unwrap();
            if STACK_DEBUG { println!("Stack: {}", if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }
            self.current_instruct += 1;
        }
        if self.name == "main" && TIME_DEBUG {
            unsafe { TIMINGS.print_timing(); }
        }
        return stack.pop().unwrap_or_else(LeBlancObject::unsafe_null);
    }


    pub fn execute(&mut self, inputs: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
        unsafe { TIMINGS.setup() }
        //inputs.clone_into(&mut self.variables);
        self.variables = inputs.to_vec();
        //println!("VARIABLES: {:?}", self.variables);
        self.current_instruct = 0;
        let mut stack = Vec::with_capacity(5);

        while self.current_instruct < self.instructions.len() as u64 {
            let instruction = self.instructions[self.current_instruct as usize];
            //println!("I can't allocate a stack");
            if DEBUG {println!("Normal Instruction: {:?}", instruction);}
            if instruction.instruct == InstructionBase::Return {
                return stack.pop().unwrap();
            }
            if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.base());
            let now = Instant::now();
            internal_handle(self, &instruction, &mut stack).unwrap();
            if STACK_DEBUG { println!("Stack: {}", if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }
            self.current_instruct += 1;
        }
        if self.name == "main" && TIME_DEBUG {
            unsafe { TIMINGS.print_timing(); }
        }
        return stack.pop().unwrap_or_else(LeBlancObject::unsafe_null);

    }

    pub fn execute_range(&mut self, left_bound: u64, right_bound: u64) -> Strawberry<LeBlancObject> {
        self.current_instruct = left_bound;
        let mut stack = Vec::with_capacity(5);
        while self.current_instruct < right_bound {
            let instruction = self.instructions[self.current_instruct as usize];
            if DEBUG {println!("Range Instruction: {:?}", instruction);}
            if instruction.instruct == InstructionBase::Return {
                return stack.pop().unwrap();
            }
            if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.base());
            let now = Instant::now();
            internal_handle(self, &instruction, &mut stack).unwrap();
            if STACK_DEBUG { println!("Stack: {}", if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }
            self.current_instruct += 1;
        }
        return stack.pop().unwrap_or_else(LeBlancObject::unsafe_null);

    }
}

impl Clone for LeblancHandle {
    fn clone(&self) -> Self {
        return LeblancHandle {
            name: self.name.clone(),
            constants: self.constants.clone(),
            variable_context: self.variable_context.clone(),
            variables: Vec::with_capacity(self.variables.capacity()),
            globals: self.globals.clone(),
            instructions: self.instructions.clone(),
            precompiled: vec![],
            current_instruct: self.current_instruct,
            null: self.null
        }
    }
}


