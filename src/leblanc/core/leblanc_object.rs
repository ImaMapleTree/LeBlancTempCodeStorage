use alloc::rc::Rc;
use std::any::Any;
use std::cell::{RefCell, RefMut};

use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::mem::{swap};
use std::pin::Pin;


use std::sync::{Arc, LockResult, MutexGuard, TryLockResult};
use fxhash::{FxHashMap, FxHashSet};
use crate::leblanc::rustblanc::strawberry::Strawberry;

use std::task::{Context, Poll};

use smol_str::SmolStr;

use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::module::Module;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::block_type::NativeBlock;
use crate::leblanc::core::native_types::class_type::ClassMeta;

use crate::leblanc::core::native_types::derived::iterator_type::{LeblancIterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::error_type::{leblanc_object_error, LeblancError};
use crate::leblanc::core::native_types::group_type::LeblancGroup;

use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::promise_type::{ArcLeblancPromise, LeblancPromise};
use crate::leblanc::core::native_types::rust_type::RustObject;
use crate::leblanc::rustblanc::Appendable;


static mut NULL: Option<Arc<Strawberry<LeBlancObject>>> = None;

static mut ERROR: Option<Arc<Strawberry<LeBlancObject>>> = None;

static mut MARKER: Option<Arc<Strawberry<LeBlancObject>>> = None;

static mut NO_ARGS: [Arc<Strawberry<LeBlancObject>>; 0] = [];

pub trait Callable {
    fn call(&mut self, method_name: &str, arguments: &mut [Arc<Strawberry<LeBlancObject>>]) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>>;
    fn call_name(&mut self, method_name: &str) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>>;
}

pub trait Reflect {
    fn reflect(&self) -> Box<dyn Any + 'static>;
}

pub trait RustDataCast<T> {
    fn clone_data(&self) -> Option<T>;
    fn ref_data(&self) -> Option<&T>;
    fn mut_data(&mut self) -> Option<&mut T>;
}

#[derive(Debug, Default)]
pub struct LeBlancObject {
    pub data: LeBlancObjectData,
    pub(crate) typing: LeBlancType,
    pub methods: Arc<FxHashSet<Method>>,
    pub members: Arc<Strawberry<FxHashMap<String, LeBlancObject>>>,
    pub context: VariableContext
}

impl LeBlancObject {
    pub fn new(data: LeBlancObjectData, typing: LeBlancType, methods: Arc<FxHashSet<Method>>, members: Arc<Strawberry<FxHashMap<String, LeBlancObject>>>, context: VariableContext) -> LeBlancObject {
        LeBlancObject {data, typing, methods, members, context}
    }

    pub fn is_error(&self) -> bool { self.typing == LeBlancType::Exception }

    pub fn null() -> LeBlancObject {
        LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Null,
            methods: Arc::new(FxHashSet::default()),
            members: Arc::new(Strawberry::new(FxHashMap::default())),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_null() -> Arc<Strawberry<LeBlancObject>> {
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

    pub fn marker() -> LeBlancObject {
        LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Marker,
            methods: Arc::new(FxHashSet::default()),
            members: Arc::new(Strawberry::new(FxHashMap::default())),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_marker() -> Arc<Strawberry<LeBlancObject>> {
        return unsafe {
            match MARKER.as_ref() {
                None => {
                    MARKER = Some(LeBlancObject::marker().to_mutex());
                    MARKER.as_ref().unwrap().clone()
                }
                Some(marker) => {
                    marker.clone()
                }
            }
        }
    }

    pub fn error() -> LeBlancObject {
        LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: LeBlancType::Exception,
            methods: Arc::new(FxHashSet::default()),
            members: Arc::new(Strawberry::new(FxHashMap::default())),
            context: VariableContext::empty()
        }
    }

    pub fn unsafe_error() -> Arc<Strawberry<LeBlancObject>> {
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

    pub fn error2() -> LeBlancObject {
        leblanc_object_error(LeblancError::default())
    }

    pub fn type_of(&self) -> LeBlancType { self.typing }


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

    pub fn swap_data(&mut self, other: &mut Self) {
        swap(&mut self.members,&mut other.members);
        swap(&mut self.data, &mut other.data);
        self.typing = other.typing
    }


    pub fn swap_rc(&mut self, other: &mut MutexGuard<LeBlancObject>) {
        swap(&mut self.members, &mut other.members);
        swap(&mut self.methods, &mut other.methods);
        swap(&mut self.data, &mut other.data);
        self.typing = other.typing;
    }

    pub fn copy_rc(&mut self, other: &mut Arc<Strawberry<LeBlancObject>>) {
        let other = other.lock();
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
            LeBlancType::String => LeBlancObjectData::String((unsafe {self.reflect().downcast_ref_unchecked::<SmolStr>()}).clone()),
            _ => LeBlancObjectData::Null
        };
        LeBlancObject::new(
            object_data,
            cast,
            self.methods.clone(),
            Arc::new(Strawberry::new(FxHashMap::default())),
            VariableContext::empty()
        )
    }

    pub fn to_leblanc_arg(&self, position: u32) -> LeBlancArgument {
        LeBlancArgument::default(self.typing, position)
    }

    pub fn _clone(&self) -> LeBlancObject {
        LeBlancObject {
            data: self.data.clone(),
            typing: self.typing,
            methods: self.methods.clone(),
            members: self.members.clone(),
            context: self.context
        }
    }

    pub fn to_mutex(self) -> Arc<Strawberry<LeBlancObject>> {
        Arc::new(Strawberry::new(self))
    }
}

impl PartialEq for LeBlancObject {
    fn eq(&self, other: &Self) -> bool {
        if self.data != other.data { return false }
        if !self.members.lock().eq(&other.members.lock()) { return false }
        self.typing == other.typing
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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
    String(SmolStr),
    Block(NativeBlock),
    Function(Box<Method>),
    Module(Module),
    Class(Box<ClassMeta>), // User defined class with ID
    Dynamic(&'static LeBlancObjectData),
    Rust(RustObject),

    Promise(ArcLeblancPromise),
    Group(LeblancGroup),

    List(LeblancList),
    Iterator(LeblancIterator),
    Error(Box<LeblancError>),
    #[default]
    Null,
}

impl Reflect for LeBlancObject {
    fn reflect(&self) -> Box<dyn Any + 'static> {
        let boxed: Box<dyn Any + 'static> = match &self.data {
            LeBlancObjectData::Char(item) => Box::new(*item),
            LeBlancObjectData::Short(item) => Box::new(*item),
            LeBlancObjectData::Int(item) => Box::new(*item),
            LeBlancObjectData::Int64(item) => Box::new(*item),
            LeBlancObjectData::Int128(item) => Box::new(*item),
            LeBlancObjectData::Arch(item) => Box::new(*item),
            LeBlancObjectData::Float(item) => Box::new(*item),
            LeBlancObjectData::Double(item) => Box::new(*item),
            LeBlancObjectData::Boolean(item) => Box::new(*item),
            LeBlancObjectData::String(item) => Box::new(item.clone()),
            LeBlancObjectData::Function(item) => Box::new(item.clone()),
            LeBlancObjectData::List(item) => Box::new(item.clone()),
            LeBlancObjectData::Iterator(item) => Box::new(item.clone()),
            _ => Box::new(0),
        };
        boxed
    }
}

impl Reflect for Arc<Strawberry<LeBlancObject>> {
    fn reflect(&self) -> Box<dyn Any + 'static> {
        return self.lock().reflect();
    }
}

pub fn passed_args_to_types(args: &Vec<Arc<Strawberry<LeBlancObject>>>) -> Vec<LeBlancArgument> {
    let mut arg_types = Vec::new();
    for i in 0..args.len() {
        arg_types.append_item( args[i].lock().to_leblanc_arg(i as u32));
    }
    arg_types

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
            LeBlancObjectData::Promise(data) => data.to_string(),
            LeBlancObjectData::Group(data) => data.to_string(),
            LeBlancObjectData::Iterator(data) => data.to_string(),
            LeBlancObjectData::Error(data) => data.to_string(),
            LeBlancObjectData::Rust(data) => data.to_string(),
            LeBlancObjectData::Null => "null".to_string()
        };
        write!(f, "{}", s)
    }
}

impl Callable for Arc<Strawberry<LeBlancObject>> {
    fn call(&mut self, method_name: &str, arguments: &mut [Arc<Strawberry<LeBlancObject>>]) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>> {
        let argument_vec = arguments.to_vec();
        let args = passed_args_to_types(&argument_vec);


        //let self_clone = Arc::clone(self);
        let method = self.lock().methods.iter().filter(|m| {
            m.matches(method_name.to_string(), &args)
        }).next().cloned();
        if method.is_none() {
            return Err(LeblancError::new("ClassMethodNotFoundException".to_string(), format!("Method {} not found in {}", method_name, self.lock().typing),vec![]).create_mutex());
        }
        Ok(method.unwrap().run( self.clone(), arguments))
    }

    fn call_name(&mut self, method_name: &str) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>> {
        if self.lock().typing == LeBlancType::Null { return Err(LeblancError::new("OperationOnNullException".to_string(), "".to_string(), vec![]).create_mutex())}
        let handle = match self.lock().methods.iter().find(|m| m.context.name == method_name) {
            None => return Err(LeblancError::new("ClassMethodNotFoundException".to_string(), format!("Method {} not found in {}", method_name, self.lock().typing),vec![]).create_mutex()),
            Some(some) => some.handle
        };
        Ok(unsafe { handle(self.clone(), &mut NO_ARGS) })
    }
}


pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for Arc<Strawberry<LeBlancObject>> {
    fn to_string(&self) -> String {
        self.clone().lock().data.to_string()
    }
}

pub trait ArcType {
    fn leblanc_type(&self) -> LeBlancType;
}

impl LeBlancObjectData {


    pub fn get_mut_inner_method(&mut self) -> Option<&mut Method> {
        match self {
            LeBlancObjectData::Function(function) => Some(function),
            _ => None
        }
    }
    pub fn get_inner_method(&self) -> Option<&Method> {
        match self {
            LeBlancObjectData::Function(function) => Some(function),
            _ => None
        }
    }
    pub fn as_i128(&self) -> i128 {
        match self {
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

    pub fn simple_operation(&self, other: &Self, _operation: LBODOperation) -> LeBlancObjectData {
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

/*impl PartialOrd for LeBlancObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}*/

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

pub trait ArcToRc<T: Clone> {
    fn to_rc(self) -> Rc<RefCell<T>>;
}

pub trait QuickUnwrap<T: Clone + Default> {
    fn arc_unwrap(self) -> T;
    fn clone_if_locked(&self) -> Arc<Strawberry<T>>;
}

impl QuickUnwrap<LeBlancObject> for Arc<Strawberry<LeBlancObject>> {
    fn arc_unwrap(self) -> LeBlancObject {
        self.force_unwrap()
    }

    fn clone_if_locked(&self) -> Arc<Strawberry<LeBlancObject>> {
        let lock_attempt = self.try_lock();
        let cloned = self.clone();
        match lock_attempt {
            Some(_) => cloned,
            None => cloned.arc_unwrap().to_mutex()
        }
    }
}