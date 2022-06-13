use std::collections::{BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter, write};
use std::hash::{Hash, Hasher};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method_handler::MethodHandle;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;

pub struct Method {
    context: MethodStore,
    //handle: MethodHandle,
    handle: fn(&LeBlancObject, &[LeBlancObject]) -> LeBlancObject,
    pub tags: BTreeSet<MethodTag>,
}

impl Method {
    pub fn new(context: MethodStore, handle: fn(&LeBlancObject, &[LeBlancObject]) -> LeBlancObject, tags: BTreeSet<MethodTag>) -> Method {
        return Method {
            context,
            handle,
            tags
        }
    }

    pub fn null() -> Method {
        return Method {
            context: MethodStore::no_args("null".to_string()),
            handle: null_func,
            tags: BTreeSet::new()
        }
    }

    pub fn default(context: MethodStore, handle: fn(&LeBlancObject, &[LeBlancObject]) -> LeBlancObject) -> Method {
        return Method::new(context, handle, BTreeSet::new());
    }

    pub fn run(&self, _self: &LeBlancObject, args: &[LeBlancObject]) -> LeBlancObject {
        return (self.handle)(_self, args);
    }

    pub fn matches(&self, name: String, arguments: Vec<LeBlancArgument>) -> bool {
        if self.context.name == name || name == "_" {
            for argument in arguments {
                if !self.context.arguments.iter()
                    .filter(|a| a.position == argument.position)
                    .any(|a| a.typing == argument.typing) {
                    return false;
                }
            }
            return true;
        }
        return false;
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
        Method::new(self.context.clone(), self.handle, self.tags.clone())
    }
}

fn null_func(_self: &LeBlancObject, args: &[LeBlancObject]) -> LeBlancObject {return LeBlancObject::null()}

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