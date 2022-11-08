use std::mem::take;
use std::sync::atomic::Ordering;
use crate::leblanc::compiler3::leblanc_type::LBType;
use crate::leblanc::compiler3::symbols::converted::FunctionArg;
use crate::leblanc::compiler3::symbols::cst::{CombinedSymbol, LCST, Scope, SubTable, SymbolType};
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Expr, Expression, Id, Ident, Statement, Stmnt};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::copystring::CopyString;

pub fn generate_symbol_table(components: Vec<Component>, old_table: Option<LCST>) -> LCST {
    let symbol_table = old_table.unwrap_or_default();
    for component in components {
        scan_component(component, symbol_table.clone());
    }
    println!("No stack overflow");
    symbol_table
}

pub fn scan_component(mut component: Component, mut table: LCST) {
    match take(&mut component.data) {
        Cmpnt::Function { header,body, tags  } => {
            let subtable = table.create_sub_table();
            let mut lock = table.lock();
            lock.create_shared_context();
            let (name, args, returns): (String, Vec<Expression>, Vec<LeBlancType>) = header.data.into_function_header().unwrap();
            let counter = lock.func_counter.fetch_add(1, Ordering::SeqCst);

            let scope = Scope::Local(lock.level);
            let subsymbols = args.into_iter().map(FunctionArg::from)
                .map(|arg| {CombinedSymbol {
                        name: arg.name,
                        scope,
                        typ: Option::from(arg.typing),
                        subsymbols: vec![],
                        location: Option::from(arg.location),
                        unique: true,
                        stype: Default::default(),
                    linked: None
                }}).collect();




            let func_symbol = CombinedSymbol {
                name: CopyString::from(name),
                scope: Scope::Intermediate,
                typ: returns.get(0).map(|ty| LBType::from(*ty)),
                subsymbols,
                location: Some(component.location),
                unique: true,
                stype: SymbolType::Function(counter),
                linked: Some(subtable.clone())
            };

            subtable.lock().create_shared_context();

            lock.add_symbol(func_symbol);
            drop(lock);
            scan_statement(body, subtable);
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

pub fn scan_statement(mut statement: Statement, mut table: LCST) {
    let location = Some(statement.location);
    match take(&mut statement.data) {
        Stmnt::Global { statement } => {
            let parent = table.get_highest_table();
            scan_statement(*statement, parent);
        }
        Stmnt::Block { statements } => {
            statements.into_iter().for_each(|s| scan_statement(s, table.clone()));
        }
        Stmnt::Line { expr } => { scan_expression(expr, table) }
        Stmnt::MultiConditional { conditionals } => {

        }
        Stmnt::While { condition, statement } => {
            let subtable = table.create_sub_table();

            let symbol = CombinedSymbol {
                name: CopyString::new("While"),
                stype: Default::default(),
                typ: None,
                subsymbols: vec![],
                location,
                unique: false,
                scope: Default::default(),
                linked: Some(subtable.clone())
            };
            table.lock().add_symbol(symbol);

            scan_statement(*statement, subtable);
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
        _ => {}
    }
}

pub fn scan_expression(mut expression: Expression, mut table: LCST) {
    let location = Some(expression.location);
    match take(&mut expression.data) {
        Expr::Break => {}
        Expr::RangeExpression { start, bound, step } => {}
        Expr::StaticMethodCall { method_name, args } => {
            scan_expression(*method_name, table.clone());
            let arg_table = table.create_sub_table();
            args.into_iter().for_each(|arg| scan_expression(arg, arg_table.clone()));
        }
        Expr::ListIndex { list, slice } => {}
        Expr::Slice { left, right } => {}
        Expr::SeriesIndex { indices } => {}
        Expr::Equality { left, comparator, right } => {}
        Expr::List { items } => {}
        Expr::ArithPlusMinusOperation { left, op, right } => {}
        Expr::ArithMulDivModOperation { left, op, right } => {}
        Expr::ExponentialOperation { left, op, right } => {}
        Expr::UnaryOperation { term, op } => {}
        Expr::IncrementDecrementOperation { term, op, postfix } => {}
        Expr::ListAssignment { list, expr } => {}
        Expr::TypedAssignment { idents, expr } => {
            if let Some(expression) = expr {
                scan_expression(*expression, table.clone())
            }
            idents.into_iter().for_each(|ident| scan_expression(ident, table.clone()));
        }
        Expr::NormalAssignment { idents, expr } => {
            scan_expression(*expr, table.clone());
            let ident_table = table.create_sub_table();
            idents.into_iter().for_each(|ident| scan_ident(ident, ident_table.clone(), None));
        }
        Expr::GroupAssignment { assignee, group } => {}
        Expr::BlockLambda { variables, block } => {}
        Expr::ExprLambda { variables, expr } => {}
        Expr::ExceptCatch { errors, variable } => {}
        Expr::TypedVariable { typing, variable } => {
            scan_ident(variable, table, Some(LBType::from(typing)));
        }
        Expr::Ident { ident } => { scan_ident(ident, table, None)}
        Expr::Constant { constant } => {}
        _ => {}
    }
}

pub fn scan_ident(mut ident: Ident, mut table: LCST, typ: Option<LBType>) {
    let location = Some(ident.location);
    let level = table.lock().level;
    let unique = typ.is_some();
    match take(&mut ident.data) {
        Id::Ident { ident } => {
            let symbol = CombinedSymbol {
                name: CopyString::from(ident),
                scope: Scope::Local(level),
                typ,
                subsymbols: vec![],
                location,
                unique,
                stype: Default::default(),
                linked: None
            };
            table.lock().add_symbol(symbol);
        }
        Id::ObjIdent { ident, attr } => {
            scan_ident(*ident, table.clone(), typ);
            scan_ident(*attr, table.create_sub_table(), None);
        }
        Id::EnumIdent { ident, kind } => {
            scan_ident(*ident, table.clone(), typ);
            scan_ident(*kind, table.create_sub_table(), None);
        }
        Id::TypedListIdent { typing } => {}
    }
}