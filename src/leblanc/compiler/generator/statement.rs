use crate::leblanc::compiler::generator::bytecode_generator::BytecodeGenerator;
use crate::leblanc::compiler::generator::expression::determine_expression;
use crate::leblanc::compiler::generator::generator_types::{GeneratedClass, GeneratedFuncHeader};
use crate::leblanc::compiler::parser::ast::{Statement, Stmnt};


impl BytecodeGenerator {
    pub fn determine_statement(&mut self, statement: &Statement) {
        match &statement.data {
            Stmnt::Global { .. } => {}
            Stmnt::Block { statements } => statements.iter().for_each(|s|
                self.determine_statement(s)
            ),
            Stmnt::Line { expr } => self.determine_expression(expr),
            Stmnt::Conditional { .. } => {}
            Stmnt::While { .. } => {}
            Stmnt::For { .. } => {}
            Stmnt::InfLoop { .. } => {}
            Stmnt::Try { .. } => {}
            Stmnt::Except { .. } => {}
            Stmnt::Return { .. } => {}
        }
    }
}