use std::any::Any;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hasher;
use std::sync::{Arc, Mutex};

use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::module::Module;
use crate::leblanc::core::native_types::block_type::NativeBlock;
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::Appendable;

static mut NULL: Option<Arc<Mutex<LeBlancObject>>> = None;

static mut ERROR: Option<Arc<Mutex<LeBlancObject>>> = None;

static mut NO_ARGS: [Arc<Mutex<LeBlancObject>>; 0] = [];

pub trait Callable {
    fn call(&mut self, method_name: &str, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>;
    fn call_name(&mut self, method_name: &str) -> Arc<Mutex<LeBlancObject>>;
}

pub trait Reflect {
    fn reflect(&self) -> Box<dyn Any + 'static>;
}

#[derive(Debug, PartialEq)]
pub struct LeBlancObject {
    pub data: LeBlancObjectData,
    pub(crate) typing: LeBlancType,
    pub methods: Arc<HashSet<Method>>,
    pub members: HashMap<String, LeBlancObject>,
    pub context: VariableContext
}

impl LeBlancObject {
    pub fn new(data: LeBlancObjectData, typing: LeBlancType, methods: Arc<HashSet<Method>>, members: HashMap<String, LeBlancObject>, context: VariableContext) -> LeBlancObject {
        return LeBlancObject {data, typing, methods, members, context};
    }


    pub fn is_error(&self) -> bool { return self.typing == LeBlancType::Exception }

    pub fn null() -> LeBlancObject {
        return LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Class(0),
            methods: Arc::new(HashSet::new()),
            members: HashMap::new(),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_null() -> Arc<Mutex<LeBlancObject>> {
        return unsafe {
            match NULL.as_ref() {
                None => {
                    NULL = Some(Arc::new(Mutex::new(LeBlancObject::null())));
                    NULL.as_ref().unwrap().clone()
                }
                Some(null) => {
                    null.clone()
                }
            }
        }
    }

    pub fn error() -> LeBlancObject {
        return LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Exception,
            methods: Arc::new(HashSet::new()),
            members: HashMap::new(),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_error() -> Arc<Mutex<LeBlancObject>> {
        return unsafe {
            match ERROR.as_ref() {
                None => {
                    ERROR = Some(Arc::new(Mutex::new(LeBlancObject::error())));
                    ERROR.as_ref().unwrap().clone()
                }
                Some(error) => {
                    error.clone()
                }
            }
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

    pub fn copy_data(&mut self, other: &Self) {
        self.members = other.members.clone();
        self.methods = other.methods.clone();
        self.typing = other.typing;
        self.data = other.data.clone();
    }

    pub fn cast(&mut self, cast: LeBlancType) -> LeBlancObject {
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
            HashMap::default(),
            VariableContext::empty()
        )
    }

    pub fn to_leblanc_arg(&self, position: u32) -> LeBlancArgument {
        return LeBlancArgument::default(self.typing.clone(), position);
    }

    pub fn _clone(&self) -> LeBlancObject {
        return LeBlancObject {
            data: self.data.clone(),
            typing: self.typing,
            methods: self.methods.clone(),
            members: self.members.clone(),
            context: self.context.clone()
        }
    }

    pub fn to_mutex(self) -> Arc<Mutex<LeBlancObject>> {
        return Arc::new(Mutex::new(self));
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    List(LeblancList),
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
            LeBlancObjectData::Function(item) => Box::new(item),
            LeBlancObjectData::List(item) => Box::new(item),
            _ => Box::new(0),
        };
        return boxed;
    }
}

impl Reflect for Arc<Mutex<LeBlancObject>> {
    fn reflect(&self) -> Box<dyn Any + 'static> {
        return self.lock().unwrap().reflect();
    }
}

pub fn passed_args_to_types(args: &Vec<Arc<Mutex<LeBlancObject>>>) -> Vec<LeBlancArgument> {
    let mut arg_types = Vec::new();
    for i in 0..args.len() {
        arg_types.append_item( args[i].lock().unwrap().to_leblanc_arg(i as u32));
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
            LeBlancObjectData::List(data) => data.to_string(),
            LeBlancObjectData::Error => "error".to_string(),
            LeBlancObjectData::Null => "null".to_string()
        };
        write!(f, "{}", s)
    }
}

impl Callable for Arc<Mutex<LeBlancObject>> {
    fn call(&mut self, method_name: &str, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
        let argument_vec = arguments.to_vec();
        let args = passed_args_to_types(&argument_vec);


        let self_clone = Arc::clone(self);
        let method = self_clone.lock().unwrap().methods.iter().filter(|m| {
            m.matches(method_name.to_string(), &args)
        }).next().cloned();
        if method.is_none() {
            return Arc::new(Mutex::new(LeBlancObject::error()));
        }
        return method.unwrap().run( self.clone(), arguments);
    }

    fn call_name(&mut self, method_name: &str) -> Arc<Mutex<LeBlancObject>> {
        let handle = self.lock().unwrap().methods.iter().find(|m| m.context.name == method_name).unwrap().handle;
        unsafe { handle(self.clone(), &mut NO_ARGS) }
    }
}


pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for Arc<Mutex<LeBlancObject>> {
    fn to_string(&self) -> String {
        self.lock().unwrap().data.to_string()
    }
}

pub trait ArcType {
    fn leblanc_type(&self) -> LeBlancType;
}

impl ArcType for Arc<Mutex<LeBlancObject>> {
    fn leblanc_type(&self) -> LeBlancType {
        return self.lock().unwrap().typing;
    }
}

impl LeBlancObjectData {
    pub fn retrieve_self_as_function(&mut self) -> Option<&mut Method> {
        return match self {
            LeBlancObjectData::Function(function) => Some(function),
            _ => None
        }
    }
    pub fn as_i128(&mut self) -> i128 {
        return match self {
            LeBlancObjectData::Char(item) => (*item).to_digit(10).unwrap() as i128,
            LeBlancObjectData::Short(item) => *item as i128,
            LeBlancObjectData::Int(item) => *item as i128,
            LeBlancObjectData::Int64(item) => *item as i128,
            LeBlancObjectData::Int128(item) => *item as i128,
            LeBlancObjectData::Arch(item) => *item as i128,
            LeBlancObjectData::Float(item) => *item as i128,
            LeBlancObjectData::Double(item) => *item as i128,
            LeBlancObjectData::Boolean(item) => *item as i128,
            _ => 0
        }
    }
}

impl PartialOrd for LeBlancObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.data.partial_cmp(&other.data);
    }
}

impl Clone for LeBlancObject {
    fn clone(&self) -> Self {
        self._clone()
    }
}