use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::future::Future;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};


use arrayvec::ArrayVec;
use smol_str::SmolStr;
use crate::leblanc::compiler::bytecode::function_bytes::FunctionBytecode;

use crate::leblanc::core::interpreter::instruction_execution::execute_instruction;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{ArcToRc, LeBlancObject, QuickUnwrap, RustDataCast, Stringify};
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::include::lib::leblanc_colored::{Color, colorize};

use crate::leblanc::rustblanc::utils::{Timings};

static DEBUG: bool = true;
static TIME_DEBUG: bool = false;
static STACK_DEBUG: bool = true;
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
    pub constants: Arc<Vec<Arc<Strawberry<LeBlancObject>>>>,
    pub variable_context: Arc<FxHashMap<String, VariableContext>>,
    pub variables: Vec<Arc<Strawberry<LeBlancObject>>>,
    pub instructions: Arc<Vec<Instruction>>,
    pub current_instruct: u64,
    pub null: bool,
    pub is_async: bool,
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
            constants: Arc::new(vec![]),
            variable_context: Arc::new(FxHashMap::default()),
            variables: vec![],
            instructions: Arc::new(vec![]),
            current_instruct: 0,
            null: true,
            is_async: false
        }
    }

    pub fn from_function_bytecode(mut bytecode: FunctionBytecode) -> LeblancHandle {
        let mut instructs: Vec<Instruction> = vec![];
        bytecode.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));
        let instructs = Arc::new(instructs);
        let constants: Vec<Arc<Strawberry<LeBlancObject>>> = bytecode.constants().into_iter().map(|constant| Arc::new(Strawberry::new(constant.to_leblanc_object()))).collect::<Vec<Arc<Strawberry<LeBlancObject>>>>();
        let variable_context = bytecode.variables();
        let name = SmolStr::new(bytecode.name());
        let context_length = variable_context.len();
        LeblancHandle {
            name,
            constants: Arc::new(constants),
            variable_context: Arc::new(variable_context),
            variables: Vec::with_capacity(context_length),
            instructions: instructs,
            current_instruct: 0,
            null: false,
            is_async: false
        }
    }

    #[inline(always)]
    pub fn execute(&mut self, inputs: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
        inputs.clone_into(&mut self.variables);
        self.current_instruct = 0;
        let mut instruction = Instruction::empty();
        let stack_trace = ArrayVec::<_, 50>::new();
        let mut stack = ArrayVec::<_, 80>::new();
        while self.current_instruct < self.instructions.len() as u64 {
            let _last_instruct = instruction;
            instruction = self.instructions[self.current_instruct as usize];
            if DEBUG {println!("{} Normal Instruction: {:?}", colorize(self.name.to_string(), Color::Blue), instruction);}
            match instruction.instruct {
                InstructionBase::Return => return stack.pop().unwrap(),
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
                    println!("Errored");
                    let mut borrowed_error = err.lock();
                    let error: &mut LeblancError = borrowed_error.data.mut_data().unwrap();
                    error.add_prior_trace(stack_trace.to_vec());
                    drop(borrowed_error);
                    unsafe { GLOBAL_SIGNAL = ExecutionSignal::Exception }
                    return err
                }
            };
            if STACK_DEBUG { println!("{} Stack: {}", colorize(self.name.to_string(), Color::Blue), if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
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

    }

    pub fn execute_range(&mut self, left_bound: u64, right_bound: u64) -> Arc<Strawberry<LeBlancObject>> {
        self.current_instruct = left_bound;
        let mut last_instruct = Instruction::empty();
        let mut instruction = Instruction::empty();
        let mut stack_trace = ArrayVec::<_, 50>::new();
        let mut stack = ArrayVec::<_, 80>::new();
        while self.current_instruct < right_bound {
            last_instruct = instruction;
            instruction = self.instructions[self.current_instruct as usize];
            if DEBUG {println!("{} Range Instruction: {:?}", colorize(self.name.to_string(), Color::Blue), instruction);}
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
            if STACK_DEBUG { println!("{} Range Stack: {}", colorize(self.name.to_string(), Color::Blue), if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            /*if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }*/
            self.current_instruct += 1;
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)

    }

    pub fn execute_instructions(&mut self, instructs: &Vec<Instruction>, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Arc<Strawberry<LeBlancObject>> {
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
    }

    pub fn execute_from_last_point(&mut self) -> LeBlancObject {
        self.execute_range(self.current_instruct, self.instructions.len() as u64).arc_unwrap()
    }

    pub fn execute_lambda(&mut self, inputs: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
        inputs.clone_into(&mut self.variables);
        let mut stack = ArrayVec::<_, 80>::new();
        let length = self.instructions.len();
        for i in 0..length {
            let instruct = self.instructions[i];
            if instruct.instruct == InstructionBase::Return { return stack.pop().unwrap() };
            execute_instruction(instruct.instruct)(self, &instruct, &mut stack);
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)

    }

    #[inline(always)]
    pub async fn execute_async(&mut self, inputs: Vec<Arc<Strawberry<LeBlancObject>>>) -> Arc<Strawberry<LeBlancObject>> {
        self.is_async = true;
        inputs.clone_into(&mut self.variables);
        self.current_instruct = 0;
        let mut instruction = Instruction::empty();
        let stack_trace = ArrayVec::<_, 50>::new();
        let mut stack: ArrayVec<Arc<Strawberry<LeBlancObject>>, 80> = ArrayVec::<_, 80>::new();
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

    }

    pub fn full_clone(&self) -> LeblancHandle {
        LeblancHandle {
            name: self.name.clone(),
            constants: Arc::new(self.constants.iter().map(|v| v.clone().arc_unwrap().to_mutex()).collect()),
            variable_context: self.variable_context.clone(),
            variables: self.variables.iter().map(|v| v.clone().arc_unwrap().to_mutex()).collect(),
            instructions: self.instructions.clone(),
            current_instruct: 0,
            null: false,
            is_async: self.is_async
        }
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
            null: self.null,
            is_async: self.is_async
        }
    }
}


pub fn dump_stack_trace(error: Arc<Strawberry<LeBlancObject>>, stack_trace: Vec<Instruction>) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed =  error.lock();
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
