use core::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;

pub struct Method {
    pub context: MethodStore,
    pub leblanc_handle: LeblancHandle,
    pub handle: fn(Arc<Mutex<LeBlancObject>>, &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>,
    pub tags: BTreeSet<MethodTag>,
}

impl Method {
    pub fn new(context: MethodStore, handle: fn(Arc<Mutex<LeBlancObject>>, &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>, tags: BTreeSet<MethodTag>) -> Method {
        return Method {
            context,
            leblanc_handle: LeblancHandle::null(),
            handle,
            tags
        }
    }

    pub fn null() -> Method {
        return Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: LeblancHandle::null(),
            handle: null_func,
            tags: BTreeSet::new()
        }
    }

    pub fn error() -> Method {
        return Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: LeblancHandle::null(),
            handle: error_func,
            tags: BTreeSet::new()
        }
    }

    pub fn no_handle(context: MethodStore, tags: BTreeSet<MethodTag>) -> Method {
        return Method::new(context, null_func, tags);
    }

    pub fn of_leblanc_handle(context: MethodStore, leblanc_handle: LeblancHandle, tags: BTreeSet<MethodTag>) -> Method {
        return Method {
            context,
            leblanc_handle,
            handle: null_func,
            tags
        }
    }

    pub fn default(context: MethodStore, handle: fn(Arc<Mutex<LeBlancObject>>, &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>) -> Method {
        return Method::new(context, handle, BTreeSet::new());
    }

    #[inline]
    pub fn run(&mut self, _self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
        return match self.leblanc_handle.null {
            false => {
                //self.leblanc_handle.variables = args.to_vec();
                self.leblanc_handle.borrow_mut().execute(Arc::new(Mutex::new(args.to_vec())))
            }
            true => (self.handle)(_self, args)
        }
    }

    pub fn run_with_vec(&mut self, _self: Arc<Mutex<LeBlancObject>>, args: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Arc<Mutex<LeBlancObject>> {
        return match self.leblanc_handle.null {
            false => {
                //self.leblanc_handle.variables = args.clone();
                self.leblanc_handle.borrow_mut().execute(Arc::new(Mutex::new(args.to_vec())))
            }
            true => (self.handle)(_self, args)
        }
    }

    pub fn run_uncloned(&self, _self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
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

    pub fn update_global(&mut self, globals: Box<Vec<Arc<Mutex<LeBlancObject>>>>) {
        self.leblanc_handle.borrow_mut().globals = globals
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
            .finish()
    }
}

impl Clone for Method {
    fn clone(&self) -> Self {
        return Method {
            context: self.context.clone(),
            handle: self.handle,
            leblanc_handle: self.leblanc_handle.clone(),
            tags: self.tags.clone()
        };
        //Method::new(self.context.clone(), self.handle, self.tags.clone())
    }
}

impl PartialOrd for Method {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.context.partial_cmp(&other.context)
    }
}

fn null_func(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {return Arc::new(Mutex::new(LeBlancObject::null()))}

fn error_func(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {return Arc::new(Mutex::new(LeBlancObject::error()))}

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