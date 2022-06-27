use fxhash::{FxHashMap, FxHashSet};
use std::sync::{Arc};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use alloc::rc::Rc;
use std::cell::RefCell;
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


#[derive(Debug, PartialEq)]
pub struct LeblancHandle {
    pub name: String,
    pub constants: Rc<Vec<Rc<RefCell<LeBlancObject>>>>,
    pub variable_context: Rc<FxHashMap<String, VariableContext>>,
    pub variables: Vec<Rc<RefCell<LeBlancObject>>>,
    pub instructions: Rc<Vec<Instruction>>,
    pub current_instruct: u64,
    pub null: bool,
}

impl LeblancHandle {
    pub fn null() -> LeblancHandle {
        return LeblancHandle {
            name: "".to_string(),
            constants: Rc::new(vec![]),
            variable_context: Rc::new(FxHashMap::default()),
            variables: vec![],
            instructions: Rc::new(vec![]),
            current_instruct: 0,
            null: true
        }
    }

    pub fn from_function_bytecode(mut bytecode: FunctionBytecode) -> LeblancHandle {
        let mut instructs: Vec<Instruction> = vec![];
        bytecode.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));
        let instructs = Rc::new(instructs);
        let constants: Vec<Rc<RefCell<LeBlancObject>>> = bytecode.constants().into_iter().map(|constant| Rc::new(RefCell::new(constant.to_leblanc_object()))).collect::<Vec<Rc<RefCell<LeBlancObject>>>>();
        let variable_context = bytecode.variables();
        let name = bytecode.name();
        let context_length = variable_context.len();
        return LeblancHandle {
            name,
            constants: Rc::new(constants),
            variable_context: Rc::new(variable_context),
            variables: Vec::with_capacity(context_length),
            instructions: instructs,
            current_instruct: 0,
            null: false
        }
    }

    pub fn execute_no_args(&mut self) -> Rc<RefCell<LeBlancObject>> {
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


    pub fn execute(&mut self, inputs: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
        unsafe { TIMINGS.setup() }
        //inputs.clone_into(&mut self.variables);
        self.variables = inputs.to_vec();
        //println!("VARIABLES: {:?}", self.variables);
        self.current_instruct = 0;
        let mut stack = Vec::with_capacity(5);

        while self.current_instruct < self.instructions.len() as u64 {
            let instruction = self.instructions[self.current_instruct as usize];
            //println!("I can't allocate a stack");
            //if DEBUG {println!("Normal Instruction: {:?}", instruction);}
            if instruction.instruct == InstructionBase::Return {
                return stack.pop().unwrap();
            }
            //if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.base());
            //let now = Instant::now();
            internal_handle(self, &instruction, &mut stack).unwrap();
            /*if STACK_DEBUG { println!("Stack: {}", if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }*/
            self.current_instruct += 1;
        }
        /*if self.name == "main" && TIME_DEBUG {
            unsafe { TIMINGS.print_timing(); }
        }*/
        return stack.pop().unwrap_or_else(LeBlancObject::unsafe_null);

    }

    pub fn execute_range(&mut self, left_bound: u64, right_bound: u64) -> Rc<RefCell<LeBlancObject>> {
        self.current_instruct = left_bound;
        let mut stack = Vec::with_capacity(5);
        while self.current_instruct < right_bound {
            let instruction = self.instructions[self.current_instruct as usize];
            //if DEBUG {println!("Range Instruction: {:?}", instruction);}
            if instruction.instruct == InstructionBase::Return {
                return stack.pop().unwrap();
            }
            if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.base());
            //let now = Instant::now();
            internal_handle(self, &instruction, &mut stack).unwrap();
            /*if STACK_DEBUG { println!("Stack: {}", if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }*/
            self.current_instruct += 1;
        }
        return stack.pop().unwrap_or_else(LeBlancObject::unsafe_null);

    }

    pub fn execute_from_last_point(&mut self) -> Rc<RefCell<LeBlancObject>> {
        return self.execute_range(self.current_instruct, self.instructions.len() as u64);
    }
}

impl Clone for LeblancHandle {
    fn clone(&self) -> Self {
        LeblancHandle {
            name: self.name.clone(),
            constants: self.constants.clone(),
            variable_context: self.variable_context.clone(),
            variables: Vec::with_capacity(self.variables.capacity()),
            instructions: self.instructions.clone(),
            current_instruct: self.current_instruct,
            null: self.null
        }
    }
}


