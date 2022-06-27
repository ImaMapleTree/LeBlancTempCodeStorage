use alloc::rc::Rc;
use std::any::Any;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hasher;
use std::intrinsics::unchecked_add;
use std::mem::take;
use std::sync::{Arc};
use fxhash::{FxHashMap, FxHashSet};

use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::module::Module;
use crate::leblanc::core::native_types::block_type::NativeBlock;
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::derived::generator_type::LeblancGenerator;
use crate::leblanc::core::native_types::derived::iterator_type::{LeblancIterable, LeblancIterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::Appendable;
use crate::leblanc::rustblanc::strawberry::{Strawberry};

static mut NULL: Option<Rc<RefCell<LeBlancObject>>> = None;

static mut ERROR: Option<Rc<RefCell<LeBlancObject>>> = None;

static mut NO_ARGS: [Rc<RefCell<LeBlancObject>>; 0] = [];

pub trait Callable {
    fn call(&mut self, method_name: &str, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>>;
    fn call_name(&mut self, method_name: &str) -> Rc<RefCell<LeBlancObject>>;
}

pub trait Reflect {
    fn reflect(&self) -> Box<dyn Any + 'static>;
}

#[derive(Debug, PartialEq)]
pub struct LeBlancObject {
    pub data: LeBlancObjectData,
    pub(crate) typing: LeBlancType,
    pub methods: Arc<FxHashSet<Method>>,
    pub members: FxHashMap<String, LeBlancObject>,
    pub context: VariableContext
}

impl LeBlancObject {
    pub fn new(data: LeBlancObjectData, typing: LeBlancType, methods: Arc<FxHashSet<Method>>, members: FxHashMap<String, LeBlancObject>, context: VariableContext) -> LeBlancObject {
        return LeBlancObject {data, typing, methods, members, context};
    }


    pub fn is_error(&self) -> bool { return self.typing == LeBlancType::Exception }

    pub fn null() -> LeBlancObject {
        return LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Class(0),
            methods: Arc::new(FxHashSet::default()),
            members: FxHashMap::default(),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_null() -> Rc<RefCell<LeBlancObject>> {
        return unsafe {
            match NULL.as_ref() {
                None => {
                    NULL = Some(LeBlancObject::null().to_mutex());
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
            methods: Arc::new(FxHashSet::default()),
            members: FxHashMap::default(),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_error() -> Rc<RefCell<LeBlancObject>> {
        return unsafe {
            match ERROR.as_ref() {
                None => {
                    ERROR = Some(LeBlancObject::error().to_mutex());
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

    pub fn move_data(&mut self, other: Self) {
        self.members = other.members;
        self.methods = other.methods;
        self.typing = other.typing;
        self.data = other.data;
    }

    pub fn copy_rc(&mut self, other: &mut Rc<RefCell<LeBlancObject>>) {
        let mut other = other.borrow_mut();
        self.members = other.members.clone();
        self.methods.clone_from(&other.methods);
        self.data = other.data.clone();
        self.typing = other.typing;
    }

    pub fn cast(&self, cast: LeBlancType) -> LeBlancObject {
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
            FxHashMap::default(),
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

    pub fn to_mutex(self) -> Rc<RefCell<LeBlancObject>> {
        return Rc::new(RefCell::new(self));
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
    Iterator(LeblancIterator),
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
            LeBlancObjectData::Iterator(item) => Box::new(item),
            _ => Box::new(0),
        };
        return boxed;
    }
}

impl Reflect for Rc<RefCell<LeBlancObject>> {
    fn reflect(&self) -> Box<dyn Any + 'static> {
        return self.borrow().reflect();
    }
}

pub fn passed_args_to_types(args: &Vec<Rc<RefCell<LeBlancObject>>>) -> Vec<LeBlancArgument> {
    let mut arg_types = Vec::new();
    for i in 0..args.len() {
        arg_types.append_item( args[i].borrow().to_leblanc_arg(i as u32));
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
            LeBlancObjectData::Iterator(data) => data.to_string(),
            LeBlancObjectData::Error => "error".to_string(),
            LeBlancObjectData::Null => "null".to_string()
        };
        write!(f, "{}", s)
    }
}

impl Callable for Rc<RefCell<LeBlancObject>> {
    fn call(&mut self, method_name: &str, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
        let argument_vec = arguments.to_vec();
        let args = passed_args_to_types(&argument_vec);


        //let self_clone = Arc::clone(self);
        let method = self.borrow().methods.iter().filter(|m| {
            m.matches(method_name.to_string(), &args)
        }).next().cloned();
        if method.is_none() {
            return LeBlancObject::unsafe_error();
        }
        return method.unwrap().run( self.clone(), arguments);
    }

    fn call_name(&mut self, method_name: &str) -> Rc<RefCell<LeBlancObject>> {
        let handle = self.borrow().methods.iter().find(|m| m.context.name == method_name).unwrap().handle;
        unsafe { handle(self.clone(), &mut NO_ARGS) }
    }
}


pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for Rc<RefCell<LeBlancObject>> {
    fn to_string(&self) -> String {
        self.clone().borrow().data.to_string()
    }
}

pub trait ArcType {
    fn leblanc_type(&self) -> LeBlancType;
}

impl LeBlancObjectData {
    pub fn get_mut_inner_method(&mut self) -> Option<&mut Method> {
        return match self {
            LeBlancObjectData::Function(function) => Some(function),
            _ => None
        }
    }
    pub fn get_inner_method(&self) -> Option<&Method> {
        return match self {
            LeBlancObjectData::Function(function) => Some(function),
            _ => None
        }
    }
    pub fn as_i128(&self) -> i128 {
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
    pub fn simple_operation(&self, other: &Self, operation: LBODOperation) -> LeBlancObjectData {
        match self {
            LeBlancObjectData::Int(data) => { LeBlancObjectData::Int(*data + other.as_i128() as i32)}
            _ => LeBlancObjectData::Null,
            /*LeBlancObjectData::Char(data) => {}
            LeBlancObjectData::Flex(data) => {}
            LeBlancObjectData::Short(data) => {}
            LeBlancObjectData::Int64(data) => {}
            LeBlancObjectData::Int128(data) => {}
            LeBlancObjectData::Arch(data) => {}
            LeBlancObjectData::Float(data) => {}
            LeBlancObjectData::Double(data) => {}
            LeBlancObjectData::Boolean(data) => {}
            LeBlancObjectData::String(data) => {}*/
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

pub enum LBODOperation {
    BinaryAddition,
    BinarySubtraction,
    BinaryMultiplication,
    BinaryDivision
}