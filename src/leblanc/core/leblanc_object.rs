use alloc::rc::Rc;
use std::any::Any;
use std::cell::{RefCell};

use std::fmt::{Debug, Display, Formatter};

use std::mem::{swap};



use std::sync::{Arc, MutexGuard};
use fxhash::{FxHashMap, FxHashSet};
use crate::leblanc::rustblanc::strawberry::Strawberry;


use lazy_static::lazy_static;

use smol_str::SmolStr;

use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::class_type::ClassMeta;

use crate::leblanc::core::native_types::derived::iterator_type::{LeblancIterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::error_type::{leblanc_object_error, LeblancError};
use crate::leblanc::core::native_types::group_type::LeblancGroup;

use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::promise_type::{ArcLeblancPromise};
use crate::leblanc::core::native_types::rust_type::RustObject;
use crate::leblanc::rustblanc::Appendable;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::core::heap::{heap, HEAP};
use crate::leblanc::core::method::Method;
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;
use crate::unsafe_vec;

lazy_static! {
    static ref LBNULL: LBObject = LeBlancObject::new(LeBlancObjectData::Null, 23, Default::default());
    static ref LBERROR: LBObject = LeBlancObject::new(LeBlancObjectData::Null, 16, Default::default());
}

pub trait Callable {
    fn call(&mut self, method_name: &str, arguments: LBObjArgs) -> Result<LBObject, LBObject>;
    fn call_name(&mut self, method_name: &str) -> Result<LBObject, LBObject>;
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
    pub typing: u32,
    pub members: UnsafeVec<LBObject>
}

impl LeBlancObject {
    #[inline(always)]
    pub fn new(data: LeBlancObjectData, typing: u32, members: UnsafeVec<LBObject>) -> LBObject {
        unsafe {HEAP.alloc(LeBlancObject {data, typing, members})}
    }

    pub fn is_error(&self) -> bool { self.typing == 16 }

    pub fn null() -> LBObject {
        heap().access().alloc(
            LeBlancObject {
                data: LeBlancObjectData::Null,
                typing: 23,
                members: UnsafeVec::default()
            }
        )
    }

    #[inline(always)]
    pub fn unsafe_null() -> LBObject {
        LBNULL.clone()
    }

    pub fn error() -> LBObject {
        heap().access().alloc(
        LeBlancObject {
            data: LeBlancObjectData::Null,
            typing: 16,
            members: UnsafeVec::default()
        })
    }

    pub fn unsafe_error() -> LBObject {
        LBERROR.clone()
    }

    pub fn error2() -> LBObject {
        leblanc_object_error(LeblancError::default())
    }

    pub fn type_of(&self) -> LeBlancType { LeBlancType::from_enum_id(self.typing as u16) }


    pub fn name_of(&self) -> String {
        match self.typing {
/*            LeBlancType::Class(_) => {
                if let LeBlancObjectData::Class(meta) = &self.data {
                    meta.name.to_string()
                } else {
                    "NOT_IMPLEMENTED".to_string()
                }
            }*/
            _ => LeBlancType::from_enum_id(self.typing as u16).to_string()
        }
    }

    pub fn copy_data(&mut self, other: &Self) {
        self.members = other.members.clone();
        self.typing = other.typing;
        self.data = other.data.clone();
    }

    pub fn move_data(&mut self, other: Self) {
        self.members = other.members;
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
        swap(&mut self.data, &mut other.data);
        self.typing = other.typing;
    }

    pub fn copy_rc(&mut self, other: &mut LBObject) {
        self.members = other.members.clone();
        self.data = other.data.clone();
        self.typing = other.typing;
    }

    pub fn cast(&self, cast: LeBlancType) -> LBObject {
        let object_data = match cast {
            LeBlancType::Char => LeBlancObjectData::Char(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Short => LeBlancObjectData::Short(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Int => LeBlancObjectData::Int(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Int64 => LeBlancObjectData::Int64(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Arch => LeBlancObjectData::Arch(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Float => LeBlancObjectData::Float(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Double => LeBlancObjectData::Double(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::Boolean => LeBlancObjectData::Boolean(unsafe {*self.reflect().downcast_ref_unchecked()}),
            LeBlancType::String => LeBlancObjectData::String((unsafe {self.reflect().downcast_ref_unchecked::<SmolStr>()}).clone()),
            _ => LeBlancObjectData::Null
        };
        LeBlancObject::new(
            object_data,
            cast.enum_id(),
            self.members.clone(),
        )
    }

    pub fn to_leblanc_arg(&self, position: u32) -> LeBlancArgument {
        LeBlancArgument::default(LeBlancType::from_enum_id(self.typing as u16), position)
    }

    pub fn _clone(&self) -> LeBlancObject {
        LeBlancObject {
            data: self.data.clone(),
            typing: self.typing,
            members: self.members.clone(),
        }
    }

    pub fn to_mutex(self) -> LBObject { heap().access().alloc(self) }
}

impl PartialEq for LeBlancObject {
    fn eq(&self, other: &Self) -> bool {
        if self.data != other.data { return false }
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
    Arch(isize),
    Float(f32), //"double32" -- internally f32
    Double(f64), // internally f64
    Boolean(bool),
    String(SmolStr),
    Class(Box<ClassMeta>), // User defined class with ID
    Rust(RustObject),

    //Promise(ArcLeblancPromise),
    //Group(LeblancGroup),

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
            LeBlancObjectData::Arch(item) => Box::new(*item),
            LeBlancObjectData::Float(item) => Box::new(*item),
            LeBlancObjectData::Double(item) => Box::new(*item),
            LeBlancObjectData::Boolean(item) => Box::new(*item),
            LeBlancObjectData::String(item) => Box::new(item.clone()),
            LeBlancObjectData::List(item) => Box::new(item.clone()),
            LeBlancObjectData::Iterator(item) => Box::new(item.clone()),
            _ => Box::new(0),
        };
        boxed
    }
}

/*impl Reflect for LBObject {
    fn reflect(&self) -> Box<dyn Any + 'static> {
        self.
    }
}*/

pub fn passed_args_to_types(args: &Vec<LBObject>) -> Vec<LeBlancArgument> {
    let mut arg_types = Vec::new();
    for i in 0..args.len() {
        arg_types.append_item( args[i].to_leblanc_arg(i as u32));
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
            LeBlancObjectData::Arch(data) => data.to_string(),
            LeBlancObjectData::Float(data) => data.to_string(),
            LeBlancObjectData::Double(data) => data.to_string(),
            LeBlancObjectData::Boolean(data) => data.to_string(),
            LeBlancObjectData::String(data) => data.to_string(),
            LeBlancObjectData::Class(data) => data.to_string(),
            LeBlancObjectData::List(data) => data.to_string(),
            LeBlancObjectData::Iterator(data) => data.to_string(),
            LeBlancObjectData::Error(data) => data.to_string(),
            LeBlancObjectData::Rust(data) => data.to_string(),
            LeBlancObjectData::Null => "null".to_string()
        };
        write!(f, "{}", s)
    }
}

pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for LBObject {
    fn to_string(&self) -> String {
        self.clone().data.to_string()
    }
}

pub trait ArcType {
    fn leblanc_type(&self) -> LeBlancType;
}

impl LeBlancObjectData {


    pub fn get_mut_inner_method(&mut self) -> Option<&mut Method> {
        match self {
            //LeBlancObjectData::Function(function) => Some(function),
            _ => None
        }
    }
    pub fn get_inner_method(&self) -> Option<&Method> {
        /*if let LeBlancObjectData::Function(func) = self {
            Some(func)
        } else { None }*/
        None
    }

    pub fn as_i128(&self) -> i128 {
        match self {
            LeBlancObjectData::Char(item) => (*item).to_digit(10).unwrap() as i128,
            LeBlancObjectData::Short(item) => *item as i128,
            LeBlancObjectData::Int(item) => *item as i128,
            LeBlancObjectData::Int64(item) => *item as i128,
            //LeBlancObjectData::Int128(item) => *item as i128,
            LeBlancObjectData::Arch(item) => *item as i128,
            LeBlancObjectData::Float(item) => *item as i128,
            LeBlancObjectData::Double(item) => *item as i128,
            LeBlancObjectData::Boolean(item) => *item as i128,
            _ => 0
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            LeBlancObjectData::Char(item) => (*item).to_digit(10).unwrap() as i64,
            LeBlancObjectData::Short(item) => *item as i64,
            LeBlancObjectData::Int(item) => *item as i64,
            LeBlancObjectData::Int64(item) => *item as i64,
            LeBlancObjectData::Arch(item) => *item as i64,
            LeBlancObjectData::Float(item) => *item as i64,
            LeBlancObjectData::Double(item) => *item as i64,
            LeBlancObjectData::Boolean(item) => *item as i64,
            _ => 0
        }
    }
}

impl Clone for LeBlancObject {
    fn clone(&self) -> Self {
        self._clone()
    }
}

unsafe impl Sync for LeBlancObject {}