use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{BraceOpen, CompileVocab, LeBlancType, Semicolon, TypedToken};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType;
use crate::LeBlancType::Function;


pub fn write_bytecode(stack: Vec<TypedToken>) {
    let mut globals: HashMap<String, u64> = HashMap::new();
    let function_map: HashMap<Function, u64> = HashMap::new();
    let mut function = Function::new("_EMPTY_".to_string());
    for token in &stack {
        if token.lang_type() == CompileVocab::KEYWORD(LBKeyword::Func) {
            function = build_function(stack[0..stack.iter().enumerate().filter(|(_, r)| r.lang_type() == CompileVocab::BOUNDARY(Semicolon) || r.lang_type() == CompileVocab::BOUNDARY(BraceOpen)).map(|(index, _)| index+1).next().unwrap()].to_vec());
        }
        else if token.global() {
            globals.insert(token.as_string(), globals.len() as u64);
        }
        else {
            let instruction = token.lang_type().as_instruction();
        }

    }
}

fn build_function(mut tokens: Vec<TypedToken>) -> Function {
    tokens.pop();
    let name_token = tokens.pop().unwrap();

    let mut func = Function::new(name_token.as_string());

    let mut next_token = tokens.pop().unwrap();
    while next_token.lang_type() != CompileVocab::BOUNDARY(BoundaryType::ParenthesisClosed) {
        if let CompileVocab::VARIABLE(lb_type) = next_token.lang_type() {
            func.add_arg(next_token.as_string(), lb_type);
        }
        next_token = tokens.pop().unwrap();
    };
    while next_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::Comma) || !next_token.lang_type().matches("boundary") {
        if let CompileVocab::TYPE(lb_type) = next_token.lang_type() {
            func.return_types.insert(func.return_types.len(), lb_type);
        }
        next_token = tokens.pop().unwrap();
    }

    return func;
}

#[derive(Clone, PartialEq, Eq)]
struct Function {
    pub name: String,
    pub arg_types: Vec<LeBlancType>,
    pub return_types: Vec<LeBlancType>,
    pub variables: HashMap<String, u64>,
    pub bytes: Vec<String>
}

impl Function {
    pub fn new(name: String) -> Function {
        return Function {
            name,
            arg_types: vec![],
            return_types: vec![],
            variables: HashMap::new(),
            bytes: vec![]
        }
    }

    pub fn add_arg(&mut self, name: String, lb_type: LeBlancType) {
        self.variables.insert(name, self.variables.len() as u64);
        self.arg_types.insert(self.arg_types.len(), lb_type);
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.arg_types.hash(state);
        self.return_types.hash(state);
    }
}