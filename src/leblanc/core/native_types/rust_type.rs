use core::fmt::{Debug, Display, Formatter};
use std::mem::take;
use std::sync::Arc;
use fxhash::{FxHashMap, FxHashSet};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::base_type::base_methods;
use crate::leblanc::rustblanc::copystring::{CopyString, CopyStringable};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::core::native_types::LeBlancType;

pub trait RustSubTrait{
    fn _clone(&self) -> Box<dyn RustType>;
    fn _debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<T> RustSubTrait for T
where
    T: 'static + Clone + Debug + RustType
{
    fn _clone(&self) -> Box<dyn RustType> {
        Box::new(self.clone())
    }

    fn _debug(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub trait RustType: RustSubTrait {}

#[derive(Clone, Debug)]
pub struct RustObject {
    pub data: Box<dyn RustType>
}

impl PartialEq for RustObject {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PartialOrd for RustObject {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}


impl RustObject {
    pub fn new(data: Box<dyn RustType>) -> RustObject {
        RustObject {
            data
        }
    }

    pub fn reflect<T>(&self) -> &T {
        unsafe {(std::ptr::addr_of!(self.data) as *const T).as_ref().unwrap()}
    }

    pub fn reflect_mut<T>(&mut self) -> &mut T {
        unsafe {(std::ptr::addr_of_mut!(self.data) as *mut T).as_mut().unwrap()}
    }
}

impl Clone for Box<dyn RustType> {
    fn clone(&self) -> Self {
        self._clone()
    }
}

pub struct RustObjectBuilder {
    name: CopyString,
    data: RustObject,
    methods: FxHashSet<Method>,
    members: FxHashMap<String, LeBlancObject>
}

impl RustType for String {}

impl Default for RustObjectBuilder {
    fn default() -> Self {
        RustObjectBuilder {
            name: CopyString::default(),
            data: RustObject::new(Box::new(String::default())),
            methods: FxHashSet::default(),
            members: FxHashMap::default()
        }
    }
}


impl RustObjectBuilder {
    pub fn name<T: Display>(&mut self, name: T) -> &mut RustObjectBuilder {
        self.name = name.to_cstring();
        self
    }

    pub fn object<T: RustType + 'static>(&mut self, data: T) -> &mut RustObjectBuilder {
        self.data = RustObject::new(Box::new(data));
        self
    }

    pub fn method(&mut self, method: Method) -> &mut RustObjectBuilder {
        self.methods.insert(method);
        self
    }

    pub fn build(&mut self) -> LeBlancObject {
        let mut methods = Arc::unwrap_or_clone(base_methods());
        self.methods.iter().cloned().for_each(|m| {methods.insert(m);});
        let name = self.name;
        LeBlancObject {
            data: LeBlancObjectData::Rust(self.data.clone()),
            typing: LeBlancType::Class(name),
            methods: Arc::new(methods),
            members: Arc::new(Strawberry::new(take(&mut self.members))),
            context: VariableContext::empty()
        }
    }
}

impl RustDataCast<RustObject> for LeBlancObjectData {
    fn clone_data(&self) -> Option<RustObject> {
        match self {
            LeBlancObjectData::Rust(obj) => Some(obj.clone()),
            _ => None,
        }
    }

    fn ref_data(&self) -> Option<&RustObject> {
        match self {
            LeBlancObjectData::Rust(obj) => Some(obj),
            _ => None,
        }
    }

    fn mut_data(&mut self) -> Option<&mut RustObject> {
        match self {
            LeBlancObjectData::Rust(obj) => Some(obj),
            _ => None,
        }
    }
}

impl Display for RustObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "RustObject")
    }
}

impl Display for Box<dyn RustType> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "RustType")
    }
}

impl Debug for Box<dyn RustType> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self._debug(f)
    }
}