use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::mem::take;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};


use arrayvec::ArrayVec;
use smol_str::SmolStr;
use crate::leblanc::compiler::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::interpreter::execution_context::ExecutionContext;
use crate::leblanc::core::interpreter::instruction_execution2::execute_instruction2;


use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{ArcToRc, LeBlancObject, QuickUnwrap, RustDataCast, Stringify};
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::include::lib::leblanc_colored::{Color, colorize};
use crate::leblanc::rustblanc::blueberry::{BlueberryVec, Quantum};
use crate::leblanc::rustblanc::lazy_store::Lazy;
use crate::leblanc::rustblanc::types::{LBObject, LeBlancStack};

use crate::leblanc::rustblanc::utils::{Timings};

static DEBUG: bool = true;
static TIME_DEBUG: bool = false;
static STACK_DEBUG: bool = false;
static mut GLOBAL_SIGNAL: ExecutionSignal = ExecutionSignal::Normal;

static mut TIMINGS: Timings = Timings { map: None};

static mut LAMBDA_HANDLE: Option<LeblancHandle> = None;

static mut COUNT: u32 = 0;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExecutionSignal {
    Normal,
    Exception,
    Terminated
}

#[derive(Debug, Default)]
pub struct LeblancHandle {
    pub name: SmolStr,
    pub constants: Vec<LBObject>,
    pub variable_context: Option<Arc<HashMap<String, VariableContext>>>,
    pub variables: Vec<LBObject>,
    pub instructions: Arc<Vec<Instruction2>>,
    pub stack: Vec<LBObject>,
    pub is_async: bool,
    pub global_index: usize,
}

impl PartialEq for LeblancHandle {
    fn eq(&self, other: &Self) -> bool {
        if !self.name.eq(&other.name) { return false }
        if !self.instructions.eq(&other.instructions) { return false }
        true
    }
}

impl LeblancHandle {
    pub fn null() -> LeblancHandle {
        LeblancHandle {
            name: SmolStr::default(),
            constants: Vec::default(),
            variable_context: None,
            variables: Vec::default(),
            instructions: Arc::new(vec![]),
            stack: Vec::with_capacity(1000),
            is_async: false,
            global_index: 0
        }
    }

    pub fn from_function_bytecode(mut bytecode: FunctionBytecode, index: usize) -> LeblancHandle {
        let mut instructs2: Vec<Instruction2> = vec![];
        bytecode.instruction_lines().into_iter().map(|line| line.to_instructions2()).for_each(|mut l| instructs2.append(&mut l));
        let constants: Vec<LBObject> = bytecode.constants().into_iter().map(|constant| constant.to_leblanc_object()).collect::<Vec<LBObject>>();
        let variable_context = bytecode.variables();
        let name = SmolStr::new(bytecode.name());
        let context_length = variable_context.len();
        let variables = vec![LeBlancObject::null(); context_length];
        println!("{} ===> {}", name, index);
        LeblancHandle {
            name,
            constants,
            variable_context: Some(Arc::new(variable_context)),
            variables,
            instructions: Arc::from(instructs2),
            stack: Vec::with_capacity(1000),
            is_async: false,
            global_index: index
        }
    }

    pub fn execute(&self, inputs: Vec<LBObject>) -> LBObject {
        ExecutionContext::new(self.global_index, self.instructions.len(), self.variables.len(), inputs).execute()
    }

    pub fn defer(&self, inputs: Vec<LBObject>) -> ExecutionContext {
        ExecutionContext::new(self.global_index, self.instructions.len(), self.variables.len(), inputs)
    }

    fn cascade_error(&self, err: LBObject) -> LBObject {
        println!("Errored: {:#?}", err);
        unsafe { GLOBAL_SIGNAL = ExecutionSignal::Exception }
        err
    }

    fn debug(&self, instruction: Instruction2) {
        if DEBUG {println!("{} Normal Instruction: {:?}", colorize(self.name.to_string(), Color::Blue), instruction);}
    }

    pub fn execute_range(&mut self, left_bound: usize, right_bound: usize) -> LBObject {
        LeBlancObject::unsafe_null()
        /*self.current_instruct = left_bound;
        let mut instruction = Instruction2::NOREF(0, []);
        let mut last_instruct = instruction;
        let mut stack_trace = ArrayVec::<_, 50>::new();
        let mut stack = ArrayVec::<_, 80>::new();
        while self.current_instruct < right_bound {
            last_instruct = instruction;
            instruction = self.instructions[self.current_instruct as usize];
            //if DEBUG {println!("{} Range Instruction: {:?}", colorize(self.name.to_string(), Color::Blue), instruction);}
            match instruction.instruct {
                InstructionBase::Return => return stack.pop().unwrap(),
                InstructionBase::CallFunction => {
                    if last_instruct.instruct == InstructionBase::LoadFunction {
                        stack_trace.push(last_instruct)
                    }
                }
                _ => {}
            }
            unsafe {if GLOBAL_SIGNAL == ExecutionSignal::Exception { return dump_stack_trace(stack.pop().unwrap(), stack_trace.to_vec()); }}
            //if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.base());
            //let now = Instant::now();
            match internal_handle(self, &instruction, &mut stack) {
                Ok(_) => {},
                Err(err) => {
                    println!("Exception");
                    let mut borrowed_error = err.lock();
                    let error: &mut LeblancError = borrowed_error.data.mut_data().unwrap();
                    error.add_prior_trace(stack_trace.to_vec());
                    drop(borrowed_error);
                    unsafe { GLOBAL_SIGNAL = ExecutionSignal::Exception }
                    return err
                }
            };
            //if STACK_DEBUG { println!("{} Range Stack: {}", colorize(self.name.to_string(), Color::Blue), if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            /*if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }*/
            self.current_instruct += 1;
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)*/

    }

    /*pub fn execute_instructions(&mut self, instructs: &Vec<Instruction>, stack: &mut ArrayVec<LBObject, 80>) -> LBObject {
        for instruct in instructs {
            if instruct.instruct == InstructionBase::Return { return stack.pop().unwrap() };
            let internal_handle = execute_instruction(instruct.base());
            match internal_handle(self, instruct, stack) {
                Ok(_) => {},
                Err(_err) => {
                }
            };
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)
    }*/

    pub fn execute_from_last_point(&mut self) -> LBObject {
        self.execute_range(0, self.instructions.len()).to_owned()
    }

    pub fn execute_lambda(&mut self, inputs: Vec<LBObject>) -> LBObject {
        LeBlancObject::unsafe_null()
        /*inputs.clone_into(&mut self.variables);
        let mut stack = ArrayVec::<_, 80>::new();
        let length = self.instructions.len();
        for i in 0..length {
            let instruct = self.instructions[i];
            if instruct.instruct == InstructionBase::Return { return stack.pop().unwrap() };
            execute_instruction(instruct.instruct)(self, &instruct, &mut stack);
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)*/

    }

    #[inline]
    pub async fn execute_async(&mut self, inputs: Vec<LBObject>) -> LBObject {
        LeBlancObject::unsafe_null()
        /*self.is_async = true;
        inputs.clone_into(&mut self.variables);
        self.current_instruct = 0;
        let mut instruction = Instruction::empty();
        let stack_trace = ArrayVec::<_, 50>::new();
        let mut stack: ArrayVec<LBObject, 80> = ArrayVec::<_, 80>::new();
        while self.current_instruct < self.instructions.len() as u64 {
            let _last_instruct = instruction;
            instruction = self.instructions[self.current_instruct as usize];
            //if DEBUG {println!("{} Async Instruction: {:?}", colorize(self.name.to_string(), Color::Blue), instruction);}
            match instruction.instruct {
                InstructionBase::Return => {
                    //println!("Exiting Async: {:?}", stack.last().unwrap());
                    return stack.pop().unwrap()
                },
                /*InstructionBase::CallFunction => {
                    if last_instruct.instruct == InstructionBase::LoadFunction {
                        stack_trace.push(last_instruct)
                    }
                }*/
                _ => {}
            }
            unsafe {if GLOBAL_SIGNAL == ExecutionSignal::Exception { return dump_stack_trace(stack.pop().unwrap(), stack_trace.to_vec()); }}
            //if TIME_DEBUG { unsafe {TIMINGS.lock(instruction.instruct.to_string())} }
            let internal_handle = execute_instruction(instruction.instruct);
            //let now = Instant::now();
            match internal_handle(self, &instruction, &mut stack) {
                Ok(_) => {},
                Err(err) => {
                    println!("Errored: {:#?}", err);
                    let mut borrowed_error = err.lock();
                    let error: &mut LeblancError = borrowed_error.data.mut_data().unwrap();
                    error.add_prior_trace(stack_trace.to_vec());
                    drop(borrowed_error);
                    unsafe { GLOBAL_SIGNAL = ExecutionSignal::Exception }
                    return err
                }
            };
            //if STACK_DEBUG { println!("{} Stack: {}", colorize(self.name.to_string(), Color::Blue), if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            /*if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }*/
            self.current_instruct += 1;
        }
        /*if self.name == "main" && TIME_DEBUG {
            unsafe { TIMINGS.print_timing(); }
        }*/
        unsafe {if GLOBAL_SIGNAL == ExecutionSignal::Exception { return dump_stack_trace(stack.pop().unwrap(), stack_trace.to_vec()); }}
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)
*/
    }
}

impl Clone for LeblancHandle {
    fn clone(&self) -> Self {
        println!("clone");
        LeblancHandle {
            name: self.name.clone(),
            constants: unsafe {self.constants.clone()},
            variable_context: self.variable_context.clone(),
            variables: vec![LeBlancObject::null(); self.variables.len()],
            instructions: self.instructions.clone(),
            stack: Vec::with_capacity(1000),
            is_async: self.is_async,
            global_index: self.global_index
        }
    }
}


pub fn dump_stack_trace(error: LBObject, stack_trace: Vec<Instruction>) -> LBObject {
    let mut borrowed =  error.clone();
    let lbe: &mut LeblancError = borrowed.data.mut_data().unwrap();
    lbe.add_prior_trace(stack_trace);
    drop(borrowed);
    error
}


impl QuickUnwrap<LeblancHandle> for Arc<Strawberry<LeblancHandle>> {
    fn arc_unwrap(self) -> LeblancHandle {
        self.force_unwrap()
    }

    fn clone_if_locked(&self) -> Arc<Strawberry<LeblancHandle>> {
        let cloned = self.clone();
        match self.locked() {
            false => cloned,
            true => Arc::new(Strawberry::new(cloned.arc_unwrap()))
        }
    }
}
