use std::mem::take;
use crate::leblanc::compiler3::symbols::symbol_table::{STCode, STFunction, SymbolTable};
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Statement, Stmnt};

pub fn generate_synbol_table(components: Vec<Component>, old_table: Option<SymbolTable>) -> SymbolTable {
    let symbol_table = old_table.unwrap_or_default();
    for component in components {

    }








    symbol_table;
}

pub fn scan_component(mut component: Component, table: &mut SymbolTable) {
    match take(&mut component.data) {
        Cmpnt::Function { header,body, tags  } => {
            let (name, args, returns) = header.data.into_function_header();
            let code = scan_statement(body, table);
        }
        Cmpnt::Class { .. } => {}
        Cmpnt::Trait { .. } => {}
        Cmpnt::Extension { .. } => {}
        Cmpnt::Property { .. } => {}
        Cmpnt::Import { .. } => {}
        Cmpnt::ExtImport { .. } => {}
        Cmpnt::Enum { .. } => {}
        Cmpnt::EnumItem { .. } => {}
        Cmpnt::Requirements { .. } => {}
        _ => {}
    }
}

pub fn scan_statement(mut statement: Statement, table: &mut SymbolTable) -> Vec<STCode> {
    match take(&mut statement.data) {
        Stmnt::Global { statement } => {
            vec![]
        }
        Stmnt::Block { statements } => {
            statements.into_iter().flat_map(|s| scan_statement(s, table)).collect()
        }
        Stmnt::Line { expr } => {}
        Stmnt::MultiConditional { .. } => {}
        Stmnt::While { condition, statement } => {

        }
        Stmnt::For { statement, variable, iterable } => {}
        Stmnt::InfLoop { statement } => {

        }
        Stmnt::Try { statement } => {

        }
        Stmnt::Except { .. } => {

        }
        Stmnt::Return { statement } => {

        }
    }
}

pub fn scan_exp