use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;

use std::time::Instant;
use arrayvec::ArrayVec;
use smol_str::SmolStr;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;

use crate::leblanc::core::interpreter::instruction_execution::execute_instruction;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast, Stringify};
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::rustblanc::lib::leblanc_colored::{Color, colorize};
use crate::leblanc::rustblanc::utils::{Timings};

static DEBUG: bool = false;
static TIME_DEBUG: bool = false;
static STACK_DEBUG: bool = false;
static mut GLOBAL_SIGNAL: ExecutionSignal = ExecutionSignal::Normal;

static mut TIMINGS: Timings = Timings { map: None};

static mut LAMBDA_HANDLE: Option<LeblancHandle> = None;



#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExecutionSignal {
    Normal,
    Exception,
    Terminated
}

#[derive(Debug, PartialEq)]
pub struct LeblancHandle {
    pub name: SmolStr,
    pub constants: Rc<Vec<Rc<RefCell<LeBlancObject>>>>,
    pub variable_context: Rc<FxHashMap<String, VariableContext>>,
    pub variables: Vec<Rc<RefCell<LeBlancObject>>>,
    pub instructions: Rc<Vec<Instruction>>,
    pub current_instruct: u64,
    pub null: bool,
}

impl LeblancHandle {
    pub fn null() -> LeblancHandle {
        LeblancHandle {
            name: SmolStr::default(),
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
        let name = SmolStr::new(bytecode.name());
        let context_length = variable_context.len();
        LeblancHandle {
            name,
            constants: Rc::new(constants),
            variable_context: Rc::new(variable_context),
            variables: Vec::with_capacity(context_length),
            instructions: instructs,
            current_instruct: 0,
            null: false
        }
    }

    #[inline(always)]
    pub fn execute(&mut self, inputs: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
        inputs.clone_into(&mut self.variables);
        self.current_instruct = 0;
        let mut instruction = Instruction::empty();
        let mut stack_trace = ArrayVec::<_, 50>::new();
        let mut stack = ArrayVec::<_, 80>::new();
        while self.current_instruct < self.instructions.len() as u64 {
            let last_instruct = instruction;
            instruction = self.instructions[self.current_instruct as usize];
            //if DEBUG {println!("{} Normal Instruction: {:?}", colorize(self.name.clone(), Color::Blue), instruction);}
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
                    let mut borrowed_error = err.borrow_mut();
                    let error: &mut LeblancError = borrowed_error.data.mut_data().unwrap();
                    error.add_prior_trace(stack_trace.to_vec());
                    drop(borrowed_error);
                    unsafe { GLOBAL_SIGNAL = ExecutionSignal::Exception }
                    return err
                }
            };
            //if STACK_DEBUG { println!("{} Stack: {}", colorize(self.name.clone(), Color::Blue), if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
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

    pub fn execute_range(&mut self, left_bound: u64, right_bound: u64) -> Rc<RefCell<LeBlancObject>> {
        self.current_instruct = left_bound;
        let mut last_instruct = Instruction::empty();
        let mut instruction = Instruction::empty();
        let mut stack_trace = ArrayVec::<_, 50>::new();
        let mut stack = ArrayVec::<_, 80>::new();
        while self.current_instruct < right_bound {
            last_instruct = instruction;
            instruction = self.instructions[self.current_instruct as usize];
            //if DEBUG {println!("{} Range Instruction: {:?}", colorize(self.name.clone(), Color::Blue), instruction);}
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
                    let mut borrowed_error = err.borrow_mut();
                    let error: &mut LeblancError = borrowed_error.data.mut_data().unwrap();
                    error.add_prior_trace(stack_trace.to_vec());
                    drop(borrowed_error);
                    unsafe { GLOBAL_SIGNAL = ExecutionSignal::Exception }
                    return err
                }
            };
            //if STACK_DEBUG { println!("{} Range Stack: {}", colorize(self.name.clone(), Color::Blue), if stack.len() > 0 {stack.get(stack.len()-1).unwrap_or(&LeBlancObject::unsafe_null()).to_string()} else { LeBlancObject::unsafe_null().to_string()});}
            /*if TIME_DEBUG {
                let duration = now.elapsed().as_secs_f64();
                unsafe { TIMINGS.add_timing(instruction.instruct.to_string(), duration); }
            }*/
            self.current_instruct += 1;
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)

    }

    pub fn execute_instructions(&mut self, instructs: &Vec<Instruction>, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Rc<RefCell<LeBlancObject>> {
        for instruct in instructs {
            if instruct.instruct == InstructionBase::Return { return stack.pop().unwrap() };
            let internal_handle = execute_instruction(instruct.base());
            match internal_handle(self, &instruct, stack) {
                Ok(_) => {},
                Err(err) => {
                }
            };
        }
        stack.pop().unwrap_or_else(LeBlancObject::unsafe_null)
    }

    pub fn execute_from_last_point(&mut self) -> LeBlancObject {
        Rc::unwrap_or_clone(self.execute_range(self.current_instruct, self.instructions.len() as u64)).into_inner()
    }

    pub fn execute_lambda(&mut self, inputs: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
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


pub fn dump_stack_trace(mut error: Rc<RefCell<LeBlancObject>>, stack_trace: Vec<Instruction>) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed =  error.borrow_mut();
    let lbe: &mut LeblancError = borrowed.data.mut_data().unwrap();
    lbe.add_prior_trace(stack_trace);
    drop(borrowed);
    error
}