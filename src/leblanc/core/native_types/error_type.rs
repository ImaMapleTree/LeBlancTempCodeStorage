
use core::fmt::{Display, Formatter};

use fxhash::{FxHashMap, FxHashSet};
use std::sync::{Arc};


use crate::leblanc::core::internal::methods::internal_class::{ _internal_to_string_};
use crate::leblanc::core::interpreter::instructions::Instruction;

use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::base_type::{base_clone_method, base_equals_method, base_expose_method, base_field_method, base_to_string_method, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::memory::heap::HeapRef;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Hash)]
pub struct LeblancError {
    name: String,
    message: String,
    stack_trace: Vec<Instruction>,
}


pub fn leblanc_object_error(error: LeblancError) -> LBObject {
    /*let mut hash_set = wild_heap().alloc_with(FxHashSet::default);
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    //hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    //hash_set.insert(Method::default(base_field_method(), _internal_field_));*/


    LeBlancObject::new(
        LeBlancObjectData::Error(Box::new(error)),
        16,
        UnsafeVec::default()
    )
}

impl Display for LeblancError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToLeblanc for LeblancError {
    fn create(&self) -> LeBlancObject {
        leblanc_object_error(self.clone())._clone()
    }
    fn create_mutex(&self) -> LBObject { leblanc_object_error(self.clone()) }
}


impl LeblancError {
    pub fn new(name: String, message: String, stack_trace: Vec<Instruction>) -> LeblancError {
        LeblancError {
            name,
            message,
            stack_trace
        }
    }

    pub fn generic() -> LeblancError {
        LeblancError {
            name: "GenericException".to_string(),
            message: "".to_string(),
            stack_trace: vec![]
        }
    }

    pub fn add_prior_trace(&mut self, mut stack_trace: Vec<Instruction>) {
        stack_trace.append(&mut self.stack_trace);
        self.stack_trace = stack_trace;
        //self.stack_trace.append(&mut stack_trace);
    }

    pub fn print_stack_trace(&self) {
        println!("STACK TRACK: {:#?}", self.stack_trace);
        /*let func_details = get_func_details(self.stack_trace.get(0).unwrap_or(&Instruction::empty()).arg as u32);
        eprintln!("{}", colorize(format!("Exception starts at {} on {}", colorize(func_details.name, Color::Bright(ColorBright::BrightYellow)), ColorString::new(&("line ".to_owned() + &self.stack_trace.get(0).unwrap_or(&Instruction::empty()).line_number.to_string())).colorize(Color::Bright(ColorBright::BrightRed)).bold()), Color::Red));
        eprintln!("   -file:///{}:{}", func_details.file.replace('\\', "/"), self.stack_trace.get(0).unwrap_or(&Instruction::empty()).line_number);
        for instruct in self.stack_trace[1..self.stack_trace.len()-1].iter() {
            if instruct.instruct == LoadFunction {
                let func_details = get_func_details(instruct.arg as u32);
                eprintln!("{}", colorize(format!("Which calls {} on {}", colorize(func_details.name, Color::Bright(ColorBright::BrightYellow)), ColorString::new(&("line ".to_owned() + &instruct.line_number.to_string())).colorize(Color::Bright(ColorBright::BrightRed)).bold()), Color::Red));
                eprintln!("   -file:///{}:{}", func_details.file.replace('\\', "/"), instruct.line_number);
            }
        }
        let func_details = get_func_details(self.stack_trace[self.stack_trace.len()-1].arg as u32);
        eprintln!("{}", colorize(format!("And finally errors in {} on {}", colorize(func_details.name, Color::Bright(ColorBright::BrightYellow)),  ColorString::new(&("line ".to_owned() + &self.stack_trace[self.stack_trace.len()-1].line_number.to_string())).colorize(Color::Bright(ColorBright::BrightRed)).bold()), Color::Red));
        eprintln!("   -file:///{}:{}", func_details.file.replace('\\', "/"), self.stack_trace[self.stack_trace.len()-1].line_number);
        println!("{}", format!("{}: {}", ColorString::new(self.name.as_str()).colorize(Color::Bright(ColorBright::BrightRed)).bold().string(), colorize(self.message.clone(), Color::Red)))*/
    }

}

impl Default for LeblancError {
    fn default() -> Self {
        LeblancError::generic()
    }
}

impl RustDataCast<LeblancError> for LeBlancObjectData {
    fn clone_data(&self) -> Option<LeblancError> {
        match self {
            LeBlancObjectData::Error(error) => Some(*error.clone()),
            _ => None,
        }
    }

    fn ref_data(&self) -> Option<&LeblancError> {
        match self {
            LeBlancObjectData::Error(error) => Some(error),
            _ => None,
        }
    }

    fn mut_data(&mut self) -> Option<&mut LeblancError> {
        match self {
            LeBlancObjectData::Error(error) => Some(error),
            _ => None,
        }
    }
}

struct FuncDetails {
    name: String,
    file: String
}

/*fn get_func_details(func_number: u32) -> FuncDetails {
    unsafe {
        let function: LBObject = get_globals()[func_number as usize].clone();
        let inner_method = function.data.get_inner_method().unwrap();
        FuncDetails {
            name: inner_method.context.name.clone(),
            file: function.context.file.to_string()
        }
    }

}
*/