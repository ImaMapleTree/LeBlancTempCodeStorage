use core::ffi::c_float;
use core::num::dec2flt::float::RawFloat;
use core::num::dec2flt::lemire::compute_float;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, write};
use serde_value::{to_value, Value};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method_handler::MethodHandle;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::module::Module;
use crate::leblanc::core::native_types::block_type::NativeBlock;
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;



pub trait Reflect {
    fn reflect(&self) -> Box<dyn Any + 'static>;
}

#[derive(Debug, Clone)]
pub struct LeBlancObject {
    pub data: LeBlancObjectData,
    typing: LeBlancType,
    pub methods: HashSet<Method>,
    pub members: HashMap<String, LeBlancObject>,
    pub context: VariableContext
}

impl LeBlancObject {
    pub fn new(data: LeBlancObjectData, typing: LeBlancType, methods: HashSet<Method>, members: HashMap<String, LeBlancObject>, context: VariableContext) -> LeBlancObject {
        return LeBlancObject {data, typing, methods, members, context};
    }

    pub fn call(&self, method_name: &str, arguments: &[LeBlancObject]) -> LeBlancObject {
        return self.methods.iter().filter(|m| {
            m.matches(method_name.to_string(), passed_args_to_types(arguments))
        }).last().unwrap().run(self, arguments);
    }

    pub fn null() -> LeBlancObject {
        return LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Class(0),
            methods: HashSet::new(),
            members: HashMap::new(),
            context: VariableContext::empty()
        }
    }

    pub fn internal_method(method: Method) -> LeBlancObject {
        return LeBlancObject {
            data: LeBlancObjectData::Function(method),
            typing: LeBlancType::Function,
            methods: HashSet::new(),
            members: HashMap::new(),
            context: VariableContext::empty()
        }
    }

    pub fn type_of(&self) -> LeBlancType { return self.typing.clone(); }


    pub fn name_of(&self) -> String {
        match self.typing {
            LeBlancType::Class(_) => {
                if let LeBlancObjectData::Class(meta) = &self.data {
                    meta.name.to_string()
                } else {
                    "NOT_IMPLEMENTED".to_string()
                }
            }
            _ => self.typing.as_str().to_string()
        }
    }

    pub fn cast(self, cast: LeBlancType) -> LeBlancObject {
        let object_data = match cast {
            LeBlancType::Char => LeBlancObjectData::Char(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Short => LeBlancObjectData::Short(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Int => LeBlancObjectData::Int(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Int64 => LeBlancObjectData::Int64(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Int128 => LeBlancObjectData::Int128(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Arch => LeBlancObjectData::Arch(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Float => LeBlancObjectData::Float(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Double => LeBlancObjectData::Double(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Boolean => LeBlancObjectData::Boolean(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::String => LeBlancObjectData::String((unsafe {self.reflect().downcast_ref_unchecked::<String>()}).clone()),
            _ => LeBlancObjectData::Null
        };
        return LeBlancObject::new(
            object_data,
            cast,
            self.methods.clone(),
            HashMap::new(),
            VariableContext::empty()
        )
    }

    pub fn to_leblanc_arg(&self, position: u32) -> LeBlancArgument {
        return LeBlancArgument::default(self.typing.clone(), position);
    }
}

#[derive(Debug, Clone)]
pub enum LeBlancObjectData {
    Flex(&'static LeBlancObjectData),
    Char(char),
    Short(i16),
    Int(i32),
    Int64(i64),
    Int128(i128),
    Arch(isize),
    Float(f32), //"double32" -- internally f32
    Double(f64), // internally f64
    Boolean(bool),
    String(String),
    Block(NativeBlock),
    Function(Method),
    Module(Module),
    Class(ClassMeta), // User defined class with ID
    Dynamic(&'static LeBlancObjectData),
    Error,
    Null,
}

impl Reflect for LeBlancObject {
    fn reflect(&self) -> Box<dyn Any + 'static> {
        let boxed: Box<dyn Any + 'static> = match self.data.clone() {
            LeBlancObjectData::Char(item) => Box::new(item),
            LeBlancObjectData::Short(item) => Box::new(item),
            LeBlancObjectData::Int(item) => Box::new(item),
            LeBlancObjectData::Int64(item) => Box::new(item),
            LeBlancObjectData::Int128(item) => Box::new(item),
            LeBlancObjectData::Arch(item) => Box::new(item),
            LeBlancObjectData::Float(item) => Box::new(item),
            LeBlancObjectData::Double(item) => Box::new(item),
            LeBlancObjectData::Boolean(item) => Box::new(item),
            LeBlancObjectData::String(item) => Box::new(item),
            _ => Box::new(0),
        };
        return boxed;
    }
}

pub fn passed_args_to_types(args: &[LeBlancObject]) -> Vec<LeBlancArgument> {
    let mut arg_types = Vec::new();
    let mut i = 0;
    for arg in args {
        arg_types.insert(arg_types.len(), arg.to_leblanc_arg(i));
        i += 1;
    }
    return arg_types;

}

impl Display for LeBlancObjectData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LeBlancObjectData::Flex(data) => data.to_string(),
            LeBlancObjectData::Char(data) => data.to_string(),
            LeBlancObjectData::Short(data) => data.to_string(),
            LeBlancObjectData::Int(data) => data.to_string(),
            LeBlancObjectData::Int64(data) => data.to_string(),
            LeBlancObjectData::Int128(data) => data.to_string(),
            LeBlancObjectData::Arch(data) => data.to_string(),
            LeBlancObjectData::Float(data) => data.to_string(),
            LeBlancObjectData::Double(data) => data.to_string(),
            LeBlancObjectData::Boolean(data) => data.to_string(),
            LeBlancObjectData::String(data) => data.to_string(),
            LeBlancObjectData::Block(data) => data.to_string(),
            LeBlancObjectData::Function(data) => data.to_string(),
            LeBlancObjectData::Module(data) => data.to_string(),
            LeBlancObjectData::Class(data) => data.to_string(),
            LeBlancObjectData::Dynamic(data) => data.to_string(),
            LeBlancObjectData::Error => "error".to_string(),
            LeBlancObjectData::Null => "null".to_string()
        };
        write!(f, "{}", s)
    }
}
