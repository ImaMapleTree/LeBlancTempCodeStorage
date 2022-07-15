
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

pub struct Method {
    pub context: MethodStore,
    pub leblanc_handle: Arc<Strawberry<LeblancHandle>>,
    pub arc_handle: Option<LeblancHandle>,
    pub handle: fn(Arc<Strawberry<LeBlancObject>>, &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>>,
    pub c_handle: fn(Arc<Strawberry<LeBlancObject>>, &mut [Arc<Strawberry<LeBlancObject>>]) -> Option<&mut LeBlancObject>,
    pub tags: BTreeSet<MethodTag>,
    pub method_type: MethodType,
}



impl Method {
    pub fn new(context: MethodStore, handle: fn(Arc<Strawberry<LeBlancObject>>, &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>>, tags: BTreeSet<MethodTag>) -> Method {
        Method {
            context,
            leblanc_handle: Arc::new(Strawberry::new(LeblancHandle::null())),
            arc_handle: None,
            handle,
            tags,
            c_handle: null_c_func,
            method_type: MethodType::InternalMethod
        }
    }

    pub fn c_method(context: MethodStore, c_handle: fn(Arc<Strawberry<LeBlancObject>>, &mut [Arc<Strawberry<LeBlancObject>>]) -> Option<&mut LeBlancObject>, tags: BTreeSet<MethodTag>) -> Method {
        Method {
            context,
            leblanc_handle: Arc::new(Strawberry::new(LeblancHandle::null())),
            arc_handle: None,
            handle: null_func,
            tags,
            c_handle,
            method_type: MethodType::LinkedMethod
        }
    }

    pub fn null() -> Method {
        let _t = String::new();
        Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: Arc::new(Strawberry::new(LeblancHandle::null())),
            arc_handle: None,
            handle: null_func,
            tags: BTreeSet::new(),
            c_handle: null_c_func,
            method_type: MethodType::InternalMethod
        }
    }

    pub fn error() -> Method {
        Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: Arc::new(Strawberry::new(LeblancHandle::null())),
            arc_handle: None,
            handle: error_func,
            tags: BTreeSet::new(),
            c_handle: null_c_func,
            method_type: MethodType::InternalMethod
        }
    }

    pub fn is_internal_method(&self) -> bool {
        matches!(self.method_type, MethodType::LinkedMethod | MethodType::InternalMethod)
    }

    pub fn no_handle(context: MethodStore, tags: BTreeSet<MethodTag>) -> Method {
        Method::new(context, null_func, tags)
    }

    pub fn of_leblanc_handle(context: MethodStore, leblanc_handle: LeblancHandle, tags: BTreeSet<MethodTag>) -> Method {
        let leblanc_handle = Arc::new(Strawberry::new(leblanc_handle));
        Method {
            context,
            leblanc_handle,
            arc_handle: None,
            handle: null_func,
            tags,
            c_handle: null_c_func,
            method_type: MethodType::DefinedMethod
        }
    }

    pub fn default(context: MethodStore, handle: fn(Arc<Strawberry<LeBlancObject>>, &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>>) -> Method {
        Method::new(context, handle, BTreeSet::new())
    }

    #[inline(always)]
    pub fn run(&mut self, _self: Arc<Strawberry<LeBlancObject>>, args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
        match self.method_type {
            MethodType::DefinedMethod => self.leblanc_handle.clone_if_locked().lock().execute(args),
            MethodType::LinkedMethod => (self.c_handle)(_self, args).unwrap()._clone().to_mutex(),
            MethodType::InternalMethod => (self.handle)(_self, args)
        }
    }


    #[inline(always)]
    pub fn run_uncloned(&self, _self: Arc<Strawberry<LeBlancObject>>, args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
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
            leblanc_handle: self.leblanc_handle.clone(),
            tags: self.tags.clone(),
            c_handle: self.c_handle.clone(),
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

fn null_func(_self: Arc<Strawberry<LeBlancObject>>, _args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {LeBlancObject::unsafe_null()}

fn error_func(_self: Arc<Strawberry<LeBlancObject>>, _args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {LeBlancObject::unsafe_error()}

fn null_c_func(_self: Arc<Strawberry<LeBlancObject>>, _args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Option<&'static mut LeBlancObject> { panic!("Function Not Implemented") }

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
    LinkedMethod,
    DefinedMethod
}