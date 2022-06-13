use std::rc::Weak;
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::relationship::Node;
use crate::{CompileVocab, TypedToken};

#[derive(Debug)]
pub struct PartialFabric {
    pub path: String,
    tokens: Vec<Node<TypedToken>>,
    imports: Vec<String>,
    errors: Vec<ErrorStub>,
}

impl PartialFabric {
    pub fn new(path: String, tokens: Vec<Node<TypedToken>>, imports: Vec<String>, errors: Vec<ErrorStub>) -> PartialFabric {
        return PartialFabric {
            path,
            tokens,
            imports,
            errors,
        }
    }

    pub fn no_path(tokens: Vec<Node<TypedToken>>, imports: Vec<String>, errors: Vec<ErrorStub>) -> PartialFabric {
        return PartialFabric::new("".to_string(), tokens, imports, errors);
    }

    pub fn tokens(&mut self) -> &mut Vec<Node<TypedToken>> { &mut self.tokens }

    pub fn errors(&self) -> &Vec<ErrorStub> { &self.errors }

    pub fn imports(&self) -> &Vec<String> { &self.imports }

    pub fn is_null(&self) -> bool { self.tokens.is_empty() }

}