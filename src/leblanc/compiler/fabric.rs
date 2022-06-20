use crate::leblanc::compiler::import::Import;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::relationship::Node;
use crate::TypedToken;
use crate::leblanc::rustblanc::hex::Hexadecimal;

#[derive(Debug)]
pub struct Fabric {
    pub path: String,
    tokens: Vec<Node<TypedToken>>,
    imports: Vec<Import>,
    errors: Vec<ErrorStub>,
    pub bytecode: Hexadecimal
}

impl Fabric {
    pub fn new(path: String, tokens: Vec<Node<TypedToken>>, imports: Vec<Import>, errors: Vec<ErrorStub>) -> Fabric {
        return Fabric {
            path,
            tokens,
            imports,
            errors,
            bytecode: Hexadecimal::empty()
        }
    }

    pub fn no_path(tokens: Vec<Node<TypedToken>>, imports: Vec<Import>, errors: Vec<ErrorStub>) -> Fabric {
        return Fabric::new("".to_string(), tokens, imports, errors);
    }

    pub fn tokens(&mut self) -> &mut Vec<Node<TypedToken>> { &mut self.tokens }

    pub fn errors(&self) -> &Vec<ErrorStub> { &self.errors }

    pub fn imports(&self) -> &Vec<Import> { &self.imports }

    pub fn is_null(&self) -> bool { self.tokens.is_empty() }

}