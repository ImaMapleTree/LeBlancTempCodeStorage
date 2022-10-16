use crate::leblanc::compiler::generator::converters::expr_to_typed_var;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Location};
use crate::leblanc::compiler::parser::ast_structs::{Function, Property, TypedVariable};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::copystring::{CopyString, CopyStringable};
use crate::leblanc::rustblanc::lazy_store::{Lazy, Strategy};
use crate::leblanc::rustblanc::path::ZCPath;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    name: CopyString,
    args: Vec<LeBlancArgument>,
    returns: Vec<LeBlancType>,
    location: Location
}

impl FunctionSignature {
    pub fn new(name: &String, args: Vec<LeBlancArgument>, returns: Vec<LeBlancType>, location: Location) -> FunctionSignature {
        FunctionSignature { name: CopyString::from(name), args, returns, location}
    }

    pub fn from_header(header: &Component) -> Result<FunctionSignature, ()> {
        if let Cmpnt::FunctionHeader { name, args, returns } = &header.data {
            let converted_args = expr_to_typed_var(args);
            let mut args: Vec<LeBlancArgument> = Vec::new();
            for (i, arg) in converted_args.iter().enumerate() {
                args.push(LeBlancArgument::default(arg.typing, i as u32));
            }
            let returns = if !returns.is_empty() { returns.clone() } else { vec![LeBlancType::Null] };
            Ok(FunctionSignature { name: CopyString::from(name), args, returns, location: header.location })
        } else {
            panic!("FunctionSignature::from_header not on header Component");
        }
    }

    pub fn from_method(method: Method, returns: Vec<LeBlancType>) -> FunctionSignature {
        return FunctionSignature::from_method_store(method.store(), returns)
    }

    fn from_method_store(method_store: &MethodStore, returns: Vec<LeBlancType>) -> FunctionSignature {
        FunctionSignature {
            name: CopyString::from(&method_store.name),
            args: method_store.arguments.clone(),
            returns,
            location: Location::builtin()
        }
    }

    pub fn args(&self) -> &Vec<LeBlancArgument> { &self.args }

    pub fn returns(&self) -> &Vec<LeBlancType> { &self.returns }

    pub fn byte_pos(&self) -> (usize, usize) {
        self.location.byte_pos
    }

    pub fn file(&self) -> ZCPath {
        self.location.file
    }

    /**
    Lazy Strategy</br>
    Compares function name and args but not file location
    **/
    pub fn lazy() -> Strategy {
        Strategy::LAZY
    }

    /**
    Standard Strategy</br>
    Compares function name and args and file location but not returns
    **/
    pub fn standard() -> Strategy {
        Strategy::STANDARD
    }

    /**
    Rust Strategy</br>
    Compares function name and args and file location and returns
    **/
    pub fn rust() -> Strategy {
        Strategy::RUST
    }
}

impl Lazy for FunctionSignature {
    fn scan(&self, other: &Self, strategy: Strategy) -> bool {
        match strategy {
            Strategy::LAZY => {
                self.name == other.name && self.args == other.args
            }
            Strategy::STANDARD => {
                self.name == other.name && self.args == other.args
                    && ((self.location.file == other.location.file) || self.location.file == ZCPath::new("__BUILTIN__") || other.location.file == ZCPath::new("__BUILTIN__"))
            }
            Strategy::RUST => self == other
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GeneratedClass {
    name: String,
    super_traits: Vec<String>,
    properties: Vec<Property>,
    functions: Vec<Function>
}

impl GeneratedClass {
    pub fn new(name: String, super_traits: Vec<String>, properties: Vec<Property>, functions: Vec<Function>) -> GeneratedClass {
        GeneratedClass {
            name,
            super_traits,
            properties,
            functions
        }
    }
}