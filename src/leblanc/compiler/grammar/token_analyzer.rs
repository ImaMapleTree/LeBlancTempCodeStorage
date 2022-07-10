use std::collections::{HashMap, HashSet};
use crate::leblanc::compiler::grammar::ast::{Cmpnt, Component, Conditional, Expr, Expression, ParseType, Statement, Stmnt};
use crate::leblanc::compiler::grammar::converters::parse_type_to_lb_type;
use crate::leblanc::compiler::grammar::grammar_structs::{ScopeType, ScopeValue, SyntaxError};

fn add_to_type_map(type_map: &mut HashMap<String, HashSet<ScopeValue>>, ident: String, value: ScopeValue) -> Result<(), SyntaxError> {
    let mut set = type_map.get_mut(&ident).unwrap_or_default();
    if set.contains(&value) {
        return Err(SyntaxError {} )
    } else {
        set.insert(value);
    }
    Ok(())
}


pub fn type_analysis(tokens: Vec<Component>) {
    let mut type_map = HashMap::new();
    let scope_value = 0;
    for token in tokens {
        match token.data {
            Cmpnt::Function { header: h, body: b, tags } => {
                analyze_header(h, scope_value, &mut type_map);
                analyze_block(b, scope_value, &mut type_map);

            },
            _ => {}
        }
    }
}

fn analyze_header(comp: Box<Component>, scope_value: u64, type_map: &mut HashMap<String, HashSet<ScopeValue>>) -> Option<SyntaxError> {
    if let Cmpnt::FunctionHeader { name: n, args: a, returns } = comp.data {
        let args: Vec<(ParseType, String)> = a.iter().map(|a| {
            let Expr::TypedVariable { typing: t, variable } = a;
            (t, variable)
        }).collect();

        let function = ScopeValue {
            scope: ScopeType::Global,
            arg_types: args.iter().cloned().map(|a| parse_type_to_lb_type(a.0)).collect(),
            types: returns.iter().cloned().map(parse_type_to_lb_type).collect()
        };

        match add_to_type_map(type_map, n, function) {
            Ok(_) => {},
            Err(err) => return Some(err)
        }

        for arg in args {
            let arg_scope = ScopeValue {
                scope: ScopeType::Local(scope_value),
                arg_types: vec![],
                types: vec![parse_type_to_lb_type(arg.0)],
            };
            match add_to_type_map(type_map, arg.1, arg_scope) {
                Ok(_) => {},
                Err(err) => return Some(err)
            }
        }
    }
    None
}

fn type_analyze_statement(statement: Statement, scope_value: u64, type_map: &mut HashMap<String, HashSet<ScopeValue>>) -> Option<SyntaxError> {
    match statement.data {
        Stmnt::Global { statement } => type_analyze_statement(*statement, scope_value, type_map),
        Stmnt::Block { statements } => {
            for statement in statements {
                match type_analyze_statement(statement, scope_value, type_map) {
                    None => {}, Some(err) => return Some(err)
                }
            }
            None
        },
        Stmnt::Line { expr } => type_analyze_expr(expr, scope_value, type_map),
        Stmnt::Conditional { conditional } => type_analyze_conditional
        Stmnt::While { condition: _c, statement } => { type_analyze_statement(*statement, scope_value, type_map) }
        Stmnt::For { variable: _v, iterable: _i, statement } => { type_analyze_statement(*statement, scope_value, type_map) }
        Stmnt::InfLoop { statement } => type_analyze_statement(*statement, scope_value, type_map),
        Stmnt::Try { statement } => type_analyze_statement(*statement, scope_value, type_map),
        Stmnt::Except { catch: _c, statement } => type_analyze_statement(*statement, scope_value, type_map),
        Stmnt::Return { statement } => type_analyze_statement(*statement, scope_value, type_map)
    }
    None
}

fn type_analyze_expr(expr: Expression, scope_value: u64, type_map: &mut HashMap<String, HashSet<ScopeValue>>) -> Option<SyntaxError> {
    match expr.data {
        Expr::TypedAssignment { idents: ids, expr } => {
            for id in ids {
                match type_analyze_type_ident(id, scope_value, type_map) {
                    None => {}
                    Some(err) => return Some(err)
                }
            } None
        }
        _ => None
    }
}

fn type_analyze_conditional(cond: Conditional, scope_value: u64, type_map: &mut HashMap<String, HashSet<ScopeValue>>) -> Option<SyntaxError> {
    let statement = match cond {
        Conditional::If { condition: _c, statement } => statement,
        Conditional::ElseIf { condition: _c, statement } => statement,
        Conditional::Else { statement } => statement,
    };
    if let Expr::TypedAssignment { idents: ids, expr } => {

    }
}

fn type_analyze_type_ident(expr: Expression, scope_value: u64, type_map: &mut HashMap<String, HashSet<ScopeValue>>, nested: bool) -> Option<SyntaxError> {
    if let Expr::TypedVariable { typing: t, variable } = expr.data {
        let scope = if nested {
            ScopeType::NestedLocal(scope_value),
        } else {
            ScopeType::
        }
        let arg_scope = ScopeValue {
            scope: ScopeType::Local(scope_value),
            arg_types: vec![],
            types: vec![parse_type_to_lb_type(t)],
        };
        match add_to_type_map(type_map, variable, arg_scope) {
            Ok(_) => {}, Err(err) => return Some(err)
        }
    }
    None
}
