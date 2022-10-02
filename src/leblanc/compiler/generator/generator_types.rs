use crate::leblanc::compiler::generator::converters::expr_to_typed_var;
use crate::leblanc::compiler::parser::ast::Cmpnt;
use crate::leblanc::compiler::parser::ast_structs::{Function, Property, TypedVariable};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GeneratedFuncHeader {
    name: String,
    args: Vec<LeBlancArgument>,
    returns: Vec<LeBlancType>
}

impl GeneratedFuncHeader {
    pub fn new(name: String, args: Vec<LeBlancArgument>, returns: Vec<LeBlancType>) -> GeneratedFuncHeader {
        GeneratedFuncHeader {
            name,
            args,
            returns
        }
    }

    pub fn from_typed_variable(name: String, args: Vec<TypedVariable>, returns: Vec<LeBlancType>) -> GeneratedFuncHeader {
        GeneratedFuncHeader {
            name,
            args: args.iter().enumerate().map(|(t, v)| LeBlancArgument::default(v.typing, t as u32)).collect(),
            returns
        }
    }

    pub fn from(header: &Cmpnt) -> GeneratedFuncHeader {
        if let Cmpnt::FunctionHeader {name, args, returns} = header {
            GeneratedFuncHeader::from_typed_variable(name.to_owned(),
                         expr_to_typed_var(args), returns.clone())
        } else {
            todo!()
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