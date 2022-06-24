
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};


use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::rustblanc::strawberry::{Either, Strawberry};
use alloc::rc::Rc;
use core::borrow::BorrowMut;
use std::cell::{BorrowMutError, RefCell, RefMut};

pub struct Method {
    pub context: MethodStore,
    pub leblanc_handle: Rc<RefCell<LeblancHandle>>,
    pub handle: fn(Rc<RefCell<LeBlancObject>>, &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>>,
    pub tags: BTreeSet<MethodTag>,
    pub method_type: MethodType,
}



impl Method {
    pub fn new(context: MethodStore, handle: fn(Rc<RefCell<LeBlancObject>>, &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>>, tags: BTreeSet<MethodTag>) -> Method {
        return Method {
            context,
            leblanc_handle: Rc::new(RefCell::new(LeblancHandle::null())),
            handle,
            tags,
            method_type: MethodType::InternalMethod
        }
    }

    pub fn null() -> Method {
        let _t = String::new();
        return Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: Rc::new(RefCell::new(LeblancHandle::null())),
            handle: null_func,
            tags: BTreeSet::new(),
            method_type: MethodType::InternalMethod
        }
    }

    pub fn error() -> Method {
        return Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: Rc::new(RefCell::new(LeblancHandle::null())),
            handle: error_func,
            tags: BTreeSet::new(),
            method_type: MethodType::InternalMethod
        }
    }

    pub fn is_internal_method(&self) -> bool { self.method_type == MethodType::InternalMethod }

    pub fn no_handle(context: MethodStore, tags: BTreeSet<MethodTag>) -> Method {
        return Method::new(context, null_func, tags);
    }

    pub fn of_leblanc_handle(context: MethodStore, leblanc_handle: LeblancHandle, tags: BTreeSet<MethodTag>) -> Method {
        let leblanc_handle = Rc::new(RefCell::new(leblanc_handle));
        return Method {
            context,
            leblanc_handle,
            handle: null_func,
            tags,
            method_type: MethodType::DefinedMethod
        }
    }

    pub fn default(context: MethodStore, handle: fn(Rc<RefCell<LeBlancObject>>, &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>>) -> Method {
        return Method::new(context, handle, BTreeSet::new());
    }

    #[inline(always)]
    pub fn run(&mut self, _self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
        unsafe {
            return match self.is_internal_method() {
                false => { match self.leblanc_handle.try_borrow_mut() {
                    Ok(mut refer) => refer.execute(args),
                    Err(err) => unsafe {&mut (*self.leblanc_handle.as_ptr())}.clone().execute(args)
                }}
                true => (self.handle)(_self, args)
            }
        }
    }

    /*#[inline(always)]
    pub fn run_with_vec(&mut self, _self: Rc<RefCell<LeBlancObject>>, args: &mut Vec<Strawberry<LeBlancObject>>) -> Rc<RefCell<LeBlancObject>> {
        let mut leblanc_handle = match self.leblanc_handle.acquire() {
            Ok(lock) => lock.get(),
            Err(mut lock) => lock.get()
        };
        let null_handle = leblanc_handle.lock().unwrap().null;
        return match null_handle {
            false => {
                //self.leblanc_handle.variables = args.clone();
                leblanc_handle.lock().unwrap().execute(args)
            }
            true => (self.handle)(_self, args)
        }
    }*/

    #[inline(always)]
    pub fn run_uncloned(&self, _self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
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
        return false;

    }

    pub fn store(&self) -> &MethodStore { &self.context }

    pub fn has_tag(&self, tag: MethodTag) -> bool { return self.tags.contains(&tag); }

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
        return true;
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
        return Method {
            context: self.context.clone(),
            handle: self.handle,
            leblanc_handle: self.leblanc_handle.clone(),
            tags: self.tags.clone(),
            method_type: self.method_type
        };
        //Method::new(self.context.clone(), self.handle, self.tags.clone())
    }
}

impl PartialOrd for Method {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.context.partial_cmp(&other.context)
    }
}

fn null_func(_self: Rc<RefCell<LeBlancObject>>, _args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {return LeBlancObject::unsafe_null()}

fn error_func(_self: Rc<RefCell<LeBlancObject>>, _args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {return LeBlancObject::unsafe_error()}

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
    return s.replacen(", ", "", 1);
}

fn tags_to_string(tags: &BTreeSet<MethodTag>) -> String {
    let mut s = String::new();
    for tag in tags {
        s += &(", ".to_owned() + &tag.to_string());
    }

    return s.replacen(", ", "", 1);
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodType {
    InternalMethod,
    DefinedMethod
}