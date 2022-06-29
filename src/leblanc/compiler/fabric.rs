use crate::leblanc::compiler::compile_types::partial_class::PartialClass;
use crate::leblanc::compiler::import::Import;
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::relationship::Node;
use crate::TypedToken;
use crate::leblanc::rustblanc::hex::Hexadecimal;

#[derive(Debug)]
pub struct Fabric {
    pub path: String,
    tokens: Vec<Node<TypedToken>>,
    imports: Vec<Import>,
    core_modules: Vec<CoreModule>,
    classes: Vec<PartialClass>,
    errors: Vec<ErrorStub>,
    pub bytecode: Hexadecimal
}

impl Fabric {
    pub fn new(path: String, tokens: Vec<Node<TypedToken>>, imports: Vec<Import>, core_modules: Vec<CoreModule>, classes: Vec<PartialClass>, errors: Vec<ErrorStub>) -> Fabric {
        Fabric {
            path,
            tokens,
            imports,
            core_modules,
            classes,
            errors,
            bytecode: Hexadecimal::empty()
        }
    }

    pub fn no_path(tokens: Vec<Node<TypedToken>>, imports: Vec<Import>, core_modules: Vec<CoreModule>, classes: Vec<PartialClass>, errors: Vec<ErrorStub>) -> Fabric {
        Fabric::new("".to_string(), tokens, imports, core_modules, classes, errors)
    }

    pub fn tokens(&mut self) -> &mut Vec<Node<TypedToken>> { &mut self.tokens }

    pub fn errors(&self) -> &Vec<ErrorStub> { &self.errors }

    pub fn imports(&self) -> &Vec<Import> { &self.imports }

    pub fn core_modules(&self) -> &Vec<CoreModule> {&self.core_modules}

    pub fn is_null(&self) -> bool { self.tokens.is_empty() }

}