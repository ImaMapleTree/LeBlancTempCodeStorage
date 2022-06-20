use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use crate::{CompilationMode, CompileVocab, Fabric, LeBlancType, TypedToken};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, FunctionType};
use crate::leblanc::core::bytecode::file_body::FileBodyBytecode;
use crate::leblanc::core::bytecode::file_header::FileHeaderBytecode;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::{LeblancBytecode, ToBytecode};
use crate::leblanc::core::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::core::internal::methods::builtins::create_partial_functions;
use crate::leblanc::core::interpreter::instructions::InstructionBase;
use crate::leblanc::core::interpreter::instructions::InstructionBase::*;
use crate::leblanc::core::partial_function::PartialFunction;
use crate::leblanc::rustblanc::{Appendable, AppendCloneable, Hexable};
use crate::leblanc::rustblanc::hex::Hexadecimal;


pub fn write_bytecode(mut stack: Vec<TypedToken>, fabric: &mut Fabric, mode: CompilationMode) {
    let globals: HashMap<String, u64> = HashMap::new();

    let mut partial_functions = create_partial_functions();
    stack.iter().filter(|t| t.lang_type() == CompileVocab::FUNCTION(FunctionType::Header)).for_each(|t| {
        let p = PartialFunction::from_token_args(&t);
        if !partial_functions.contains(&p) {
            partial_functions.push(p);
        }
    });


    let mut functions: Vec<Function> = vec![];
    let mut function = Function::new("__GLOBAL__".to_string());
    let mut last_instruction = Zero;
    let mut instruction = Zero;
    let mut last_line = 0;
    let line_bytes = last_line.to_hex(4);

    let mut instruction_bytes = InstructionBytecode::new();
    stack.reverse();


    while !stack.is_empty() {
        let token_ref = &stack[stack.len()-1];
        if token_ref.token().line_number() != last_line {
            last_line = token_ref.token().line_number();

            let generated = instruction_bytes.generate();
            if generated.len() > 4 {
                function.add_bytes(generated);
            }

            instruction_bytes = InstructionBytecode::new();
            instruction_bytes.set_line_number(last_line);
        }


        if token_ref.lang_type() == CompileVocab::KEYWORD(LBKeyword::Func) {
            functions.push(function);
            function = build_function(&mut stack);
        }
        else {
            let token = stack.pop().unwrap();
            last_instruction = instruction;
            instruction = InstructionBase::from_compile_vocab(&token);

            let mut arg_byte = Hexadecimal::from_string("0000".to_string());
            if instruction == StoreUndefined {
                instruction = match last_instruction {
                    LoadGlobal => StoreGlobal,
                    _ => StoreLocal
                };
                arg_byte = instruction_bytes.remove().1;

            } else if instruction == LoadConstant {
                arg_byte = (function.constants.len() as u16).to_hex(2);
                function.constants.append_clone(&token);
            } else if instruction == LoadLocal {
                arg_byte = function.variable(token.as_string()).to_hex(2);
            } else if instruction == CallFunction {
                let index_partial: Option<(usize, PartialFunction)> = partial_functions.iter().cloned().enumerate().filter(|(index, p)| *p == PartialFunction::from_token_args(&token)).next();
                if index_partial.is_none() {
                    println!("{:#?}", partial_functions);
                    println!("{:?}", PartialFunction::from_token_args(&token));
                    panic!("This should be an actual error");
                } else {
                    let index = index_partial.as_ref().unwrap().0;
                    let partial_function = index_partial.unwrap().1;
                    instruction_bytes.add_instruction(LoadFunction.to_hex(2), index.to_hex(2));
                    arg_byte = (partial_function.args.len() as u16).to_hex(2);
                }
            }

            let instruct_byte = instruction.to_hex(2);

            if instruction != Zero {
                instruction_bytes.add_instruction(instruct_byte, arg_byte);
            } else {
                instruction = last_instruction;
            }
        }

    }
    println!("partial functions: {:#?}", partial_functions);
    let generated = instruction_bytes.generate();
    if generated.len() != 4 {
        function.add_bytes(generated);
    }
    functions.push(function);

    let mut header = FileHeaderBytecode::new();
    for import in fabric.imports() {
        header.add_import_name(&import.source);
    }

    let mut body = FileBodyBytecode::new();
    for function in functions {
        let mut function_bytecode = FunctionBytecode::new();
        function_bytecode.set_name(function.name);
        for constant in function.constants {
            let native_type = constant.lang_type().extract_native_type().clone();
            //println!("{:?}, {} | {}", constant, native_type, native_type.transform(constant.as_string()));
            function_bytecode.add_constant(native_type.transform(constant.as_string()), native_type.enum_id() as u16);
        }
        for variable in function.variables {
            function_bytecode.add_variable(variable.0, variable.1 as u32);
        }
        for bytearray in function.bytearrays {
            function_bytecode.add_instruction_line(bytearray);
        }
        body.add_function(function_bytecode);
    }

    let mut bytecode = LeblancBytecode::new(header, body);
    //println!("{:?}", bytecode.generate());
    let file = File::options().truncate(true).write(true).create(true).open(fabric.path.replace(".lb", ".lbbc"));
    let generated = bytecode.generate();

    fabric.bytecode = generated;
    if mode != CompilationMode::Realtime {
        file.unwrap().write_all(&hex::decode(fabric.bytecode.to_string()).unwrap()).unwrap();
    }
}

fn build_function(tokens: &mut Vec<TypedToken>) -> Function {
    //println!("Tokens: {:#?}", tokens);
    tokens.pop();
    let name_token = tokens.pop().unwrap();

    let mut func = Function::new(name_token.as_string());

    let mut next_token = tokens.pop().unwrap();
    while next_token.lang_type() != CompileVocab::BOUNDARY(BoundaryType::ParenthesisClosed) {
        //println!("Next token: {:?}", next_token);
        if let CompileVocab::VARIABLE(lb_type) = next_token.lang_type() {
            func.add_arg(next_token.as_string(), lb_type);
        }
        next_token = tokens.pop().unwrap();
    };
    while next_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::Comma) || !next_token.lang_type().matches("boundary") {
        if let CompileVocab::TYPE(lb_type) = next_token.lang_type() {
            func.return_types.append_item(lb_type);
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
    pub constants: Vec<TypedToken>,
    pub bytearrays: Vec<Hexadecimal>
}

impl Function {
    pub fn new(name: String) -> Function {
        return Function {
            name,
            arg_types: vec![],
            return_types: vec![],
            variables: HashMap::new(),
            constants: vec![],
            bytearrays: vec![]
        }
    }

    pub fn add_arg(&mut self, name: String, lb_type: LeBlancType) {
        self.variables.insert(name, self.variables.len() as u64);
        self.arg_types.append_item(lb_type);
    }

    pub fn add_bytes(&mut self, bytes: Hexadecimal) {
        self.bytearrays.append_item( bytes);
    }

    pub fn variable(&mut self, name: String) -> u64 {
        if self.variables.contains_key(&name) {
            return *self.variables.get(&name).unwrap();
        }
        let value = self.variables.len() as u64;
        self.variables.insert(name, value);
        return value;
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.arg_types.hash(state);
        self.return_types.hash(state);
    }
}