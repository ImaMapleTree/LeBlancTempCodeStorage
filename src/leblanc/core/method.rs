use std::collections::{BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter, write};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, Weak};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method_handler::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_handler::MethodHandle;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::LeBlancType;

pub struct Method {
    context: MethodStore,
    //handle: MethodHandle,
    leblanc_handle: Option<LeblancHandle>,
    handle: fn(Arc<Mutex<LeBlancObject>>, &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>,
    pub tags: BTreeSet<MethodTag>,
}

impl Method {
    pub fn new(context: MethodStore, handle: fn(Arc<Mutex<LeBlancObject>>, &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>, tags: BTreeSet<MethodTag>) -> Method {
        return Method {
            context,
            leblanc_handle: None,
            handle,
            tags
        }
    }

    pub fn null() -> Method {
        return Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: None,
            handle: null_func,
            tags: BTreeSet::new()
        }
    }

    pub fn error() -> Method {
        return Method {
            context: MethodStore::no_args("null".to_string()),
            leblanc_handle: None,
            handle: error_func,
            tags: BTreeSet::new()
        }
    }

    pub fn of_leblanc_handle(context: MethodStore, leblanc_handle: LeblancHandle, tags: BTreeSet<MethodTag>) -> Method {
        return Method {
            context,
            leblanc_handle: Some(leblanc_handle),
            handle: null_func,
            tags
        }
    }

    pub fn default(context: MethodStore, handle: fn(Arc<Mutex<LeBlancObject>>, &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>>) -> Method {
        return Method::new(context, handle, BTreeSet::new());
    }

    pub fn run(&self, _self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
        if self.leblanc_handle.is_some() {return self.leblanc_handle.as_ref().unwrap().clone().execute(args.to_vec())}
        return (self.handle)(_self, args);
    }

    pub fn matches(&self, name: String, arguments: Vec<LeBlancArgument>) -> bool {
        if self.context.name == name || name == "_" {
            for argument in arguments {
                if !self.context.arguments.iter()
                    .filter(|a| a.position == argument.position)
                    .any(|a| a.typing == argument.typing || argument.typing == LeBlancType::Flex) {
                    return false;
                }
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
            .finish()
    }
}

impl Clone for Method {
    fn clone(&self) -> Self {
        let lb_handle = if self.leblanc_handle.is_some() {
            Some(self.leblanc_handle.as_ref().unwrap().clone())
        } else {
            None
        };

        return Method {
            context: self.context.clone(),
            handle: self.handle,
            leblanc_handle: lb_handle,
            tags: self.tags.clone()
        };
        //Method::new(self.context.clone(), self.handle, self.tags.clone())
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