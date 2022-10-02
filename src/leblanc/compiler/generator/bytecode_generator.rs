use crate::leblanc::compiler::bytecode::file_body::FileBodyBytecode;
use crate::leblanc::compiler::bytecode::file_header::FileHeaderBytecode;
use crate::leblanc::compiler::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::compiler::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::compiler::generator::generator_types::{GeneratedClass, GeneratedFuncHeader};
use crate::leblanc::compiler::parser::ast::Expression;
use crate::leblanc::rustblanc::component_map::ComponentMap;

#[derive(Default, Debug)]
pub struct BytecodeGenerator {
    header: FileHeaderBytecode,
    body: FileBodyBytecode,
    function: FunctionBytecode,
    line: InstructionBytecode,
    pub class_map: ComponentMap<GeneratedClass>,
    pub func_map: ComponentMap<GeneratedFuncHeader>
}

impl BytecodeGenerator {
    pub fn bytecode_range_expr(&mut self, start: &Box<Expression>, bound: &Box<Expression>, step: &Box<Expression>) {

    }
}