use alloc::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::compiler::compile_types::partial_function::PartialFunction;

use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Module {
    pub path: String
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreModule {
    pub name: String,
    pub methods: Vec<ModuleMethod>,
    pub exp_methods: Box<Vec<Box<ModuleMethod>>>
}

impl CoreModule {
    pub fn new(name: String, methods: Vec<ModuleMethod>) -> CoreModule {
        CoreModule {
            name,
            methods,
            exp_methods: Box::new(vec![])
        }
    }
    pub fn add_method(&mut self, method: ModuleMethod) {
        self.methods.push(method);
    }
    pub fn add_method_box(&mut self, method: ModuleMethod) {
        self.exp_methods.push(Box::new(method));
    }

    pub fn methods_as_partials(&self) -> Vec<PartialFunction> {
        self.methods.iter().map(|method| PartialFunction::from_method(method.method.clone(), method.returns.clone())).collect()
    }
    pub fn methods_as_objects(&self) -> Vec<Arc<Strawberry<LeBlancObject>>> {
        self.methods.iter().map(|method| internal_method(method.method.clone()).to_mutex()).collect()
    }
}

impl Default for CoreModule {
    fn default() -> Self {
        CoreModule::new("".to_string(), vec![])
    }
}

unsafe impl Send for CoreModule {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModuleMethod {
    pub method: Method,
    pub returns: Vec<LeBlancType>
}

impl ModuleMethod {
    pub fn new(method: Method, returns: Vec<LeBlancType>) -> ModuleMethod {
        ModuleMethod {
            method,
            returns
        }
    }
}