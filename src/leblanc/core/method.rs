
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};


use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{ArcToRc, LeBlancObject, QuickUnwrap};
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;

use alloc::rc::Rc;

use std::cell::{RefCell};
use std::mem::take;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::interpreter::leblanc_runner::get_handles;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::{LBFunctionHandle, LBObject};

pub struct Method {
    pub context: MethodStore,
    pub leblanc_handle: &'static mut LeblancHandle,
    pub arc_handle: Option<LeblancHandle>,
    pub handle: LBFunctionHandle,
    pub tags: BTreeSet<MethodTag>,
    pub method_type: MethodType,
}



impl Method {
    pub fn new(context: MethodStore, handle: LBFunctionHandle, tags: BTreeSet<MethodTag>) -> Method {
        Method {
            context,
            leblanc_handle: get_handles().get_mut(0).unwrap(),
            arc_handle: None,
            handle,
            tags,
            method_type: MethodType::InternalMethod
        }
    }

    pub fn null() -> Method {
        let _t = String::new();
        Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: get_handles().get_mut(0).unwrap(),
            arc_handle: None,
            handle: null_func,
            tags: BTreeSet::new(),
            method_type: MethodType::InternalMethod
        }
    }

    pub fn error() -> Method {
        Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: get_handles().get_mut(0).unwrap(),
            arc_handle: None,
            handle: error_func,
            tags: BTreeSet::new(),
            method_type: MethodType::InternalMethod
        }
    }

    pub fn is_internal_method(&self) -> bool {
        matches!(self.method_type, MethodType::InternalMethod)
    }

    pub fn no_handle(context: MethodStore, tags: BTreeSet<MethodTag>) -> Method {
        Method::new(context, null_func, tags)
    }

    pub fn of_leblanc_handle(context: MethodStore, handle_index: usize, tags: BTreeSet<MethodTag>) -> Method {
        Method {
            context,
            leblanc_handle: get_handles().get_mut(handle_index).unwrap(),
            arc_handle: None,
            handle: null_func,
            tags,
            method_type: MethodType::DefinedMethod
        }
    }

    pub fn default(context: MethodStore, handle: LBFunctionHandle) -> Method {
        Method::new(context, handle, BTreeSet::new())
    }

    #[inline(always)]
    pub fn run(&self, _self: LBObject, args: Vec<LBObject>) -> LBObject {
        match self.method_type {
            MethodType::DefinedMethod => self.leblanc_handle.execute(args),
            MethodType::InternalMethod => (self.handle)(_self, args)
        }
    }


    #[inline(always)]
    pub fn run_uncloned(&self, _self: LBObject, args: Vec<LBObject>) -> LBObject {
        (self.handle)(_self, args)
    }

    pub fn matches(&self, name: String, arguments: &Vec<LeBlancArgument>) -> bool {
        if self.context.name == "call" {
            return true;
        }
        if self.context.name == name || name == "_" {
            for argument in arguments {
                if !self.context.arguments.contains(argument) { return false; }
            }
            return true;
        }
        false

    }

    pub fn store(&self) -> &MethodStore { &self.context }

    pub fn has_tag(&self, tag: MethodTag) -> bool { self.tags.contains(&tag) }

    pub fn has_tags(&self, tags: Vec<MethodTag>) -> bool {
        return tags.iter().all(|tag| self.tags.contains(tag));
    }
}

impl Eq for Method {}


impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        if self.context != other.context || self.tags != other.tags {
            return false;
        }
        true
    }
}

impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.hash(state);
        self.tags.hash(state);
    }
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("context", &self.context)
            .field("tags", &self.tags)
            .field("handle", &self.leblanc_handle)
            .finish()
    }
}

impl Clone for Method {
    fn clone(&self) -> Self {
        Method {
            context: self.context.clone(),
            handle: self.handle,
            leblanc_handle: get_handles().get_mut(self.leblanc_handle.global_index).unwrap(),
            tags: self.tags.clone(),
            method_type: self.method_type,
            arc_handle: None
        }
        //Method::new(self.context.clone(), self.handle, self.tags.clone())
    }
}

impl PartialOrd for Method {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.context.partial_cmp(&other.context)
    }
}

fn null_func(_self: LBObject, _args: Vec<LBObject>) -> LBObject {LeBlancObject::unsafe_null()}

fn error_func(_self: LBObject, _args: Vec<LBObject>) -> LBObject{LeBlancObject::unsafe_error()}

fn null_c_func(_self: LBObject, _args: Vec<LBObject>) -> Option<&'static mut LeBlancObject> { panic!("Function Not Implemented") }

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Method({}, argtypes={}, Tags={})", self.context.name, args_to_string(&self.context.arguments), tags_to_string(&self.tags))
    }
}

fn args_to_string(args: &Vec<LeBlancArgument>) -> String {
    let mut s = String::new();
    for arg in args {
        s += &(", ".to_owned() + &arg.to_string());
    }
    s.replacen(", ", "", 1)
}

fn tags_to_string(tags: &BTreeSet<MethodTag>) -> String {
    let mut s = String::new();
    for tag in tags {
        s += &(", ".to_owned() + &tag.to_string());
    }

    s.replacen(", ", "", 1)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodType {
    InternalMethod,
    DefinedMethod
}