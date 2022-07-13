use std::collections::HashMap;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Statement, Stmnt};
use crate::leblanc::compiler::parser::parse_structs::{ScopeSet, SyntaxError};

/*pub fn type_analysis(tokens: Vec<Component>, type_map: HashMap<String, ScopeSet>) {
    for token in tokens {
        match token.data {
            Cmpnt::Function { header: _h, body: b, tags } => {

            }
            _ => {}
        }
    }
}

fn type_analy_stmnt(statement: Box<Statement>, type_map: &mut HashMap<String, ScopeSet>) -> Option<SyntaxError> {
    match statement.data {
        Stmnt::Global { statement } => type_analy_stmnt(statement, type_map),
        Stmnt::Block { statements } => type_analy_stmnt(statement, type_map),
        Stmnt::Line { expr } => {}
        Stmnt::Conditional { conditional } => {}
        Stmnt::While { condition: _c, statement } => {}
        Stmnt::For { variable: _v, iterable: _i, statement } => {}
        Stmnt::InfLoop { statement } => {}
        Stmnt::Try { statement } => {}
        Stmnt::Except { catch: _c, statement } => {}
        Stmnt::Return { statement } => {}
    }
}

fn type_analy_expr(statement: Box<Statement>, type_map: &mut HashMap<String, ScopeSet>) -> Option<SyntaxError> {


}*/