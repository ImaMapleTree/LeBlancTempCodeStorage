use alloc::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap};
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Conditional, Expr, Expression, Statement, Stmnt};
use crate::leblanc::compiler::parser::import_manager::CompiledImport;
use crate::leblanc::compiler::parser::parse_structs::{IdentStore, ScopeSet, ScopeTrack, ScopeType, ScopeValue, SyntaxError};
use crate::leblanc::core::internal::methods::builtins::create_partial_functions;
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::core::native_types::type_value;
use crate::leblanc::core::native_types::LeBlancType;

static mut VARIABLE_ID: usize = 0;
static mut GLOBAL_ID: usize = 0;

fn vid_pp() -> usize {
    unsafe {
        let current = VARIABLE_ID;
        VARIABLE_ID += 1;
        current
    }
}

fn gid_pp() -> usize {
    unsafe {
        let current = GLOBAL_ID;
        GLOBAL_ID += 1;
        current
    }
}

fn add_to_type_map(type_map: &mut HashMap<IdentStore, ScopeSet>, ident: IdentStore, value: ScopeValue) -> Result<(), SyntaxError> {
    let set = type_map.get_mut(&ident);

    match set {
        None => {
            let mut set = ScopeSet::new();
            set.insert(value);
            type_map.insert(ident, set);
        }
        Some(set) => {
            println!("Checking value: {:?}", value);
            if set.invalid_in_scope(&value) {
                println!("There was a nono!!: {:#?} | {:?}", ident, value.types);
                return Err(SyntaxError {} )
            } else {
                set.insert(value);
            }
        }
    }
    Ok(())
}


pub fn declaration_analysis(modules: &mut Vec<CompiledImport>) -> HashMap<String, HashMap<IdentStore, ScopeSet>> {
    unsafe { GLOBAL_ID = create_partial_functions().len()}
    let mut import_map = HashMap::new();

    for import in modules {
        import_map.insert(import.name.clone(), HashMap::new());
        let type_map = import_map.get_mut(&import.name).unwrap();
        let scope = Rc::new(RefCell::new(ScopeTrack::default()));
        for mut token in &mut import.components {
            match &mut token.data {
                Cmpnt::Function { header: h, body: b, tags } => {
                    let st = Rc::new(RefCell::new(scope.borrow_mut().bump()));
                    let result1 = analyze_header(h, st.clone(), type_map);
                    let result2 = dec_analy_stmnt(b.clone(), st, type_map);
                    println!("Result1: {:?}", result1);
                    println!("Result2: {:?}", result2);

                },
                _ => {}
            }
        }
    }

    import_map
}

fn analyze_header(comp: &mut Box<Component>, scope_type: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<IdentStore, ScopeSet>) -> Option<SyntaxError> {
    unsafe { VARIABLE_ID = 0 }
    if let Cmpnt::FunctionHeader { name: n, args: a, returns } = &comp.data {
        let args: Vec<(LeBlancType, String)> = a.iter().map(|a| {
            let(t, variable) = if let Expr::TypedVariable { typing: t, variable } = a.clone().data {(t, variable)} else { (LeBlancType::Null, String::default()) };
            (t, variable)
        }).collect();

        let function = ScopeValue {
            scope: ScopeType::Global,
            arg_types: args.iter().map(|a| a.0).collect(),
            types: returns.clone(),
            id: gid_pp()
        };

        match add_to_type_map(type_map, IdentStore::Function(n.to_string(), function.arg_types.clone()), function) {
            Ok(_) => {},
            Err(err) => return Some(err)
        }

        for arg in args {
            let arg_scope = ScopeValue {
                scope: scope_type.borrow().get_scope_type(),
                arg_types: vec![],
                types: vec![arg.0],
                id: vid_pp()
            };
            match add_to_type_map(type_map, IdentStore::Variable(arg.1), arg_scope) {
                Ok(_) => {},
                Err(err) => return Some(err)
            }
        }
    }
    None
}

fn dec_analy_stmnt(statement: Statement, mut scope_type: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<IdentStore, ScopeSet>) -> Option<SyntaxError> {
    match statement.data {
        Stmnt::Global { statement } => dec_analy_stmnt(*statement, Rc::new(RefCell::new(ScopeTrack::default())), type_map),
        Stmnt::Block { statements } => {
            for statement in statements {
                match dec_analy_stmnt(statement, scope_type.clone(), type_map) {
                    None => {}, Some(err) => return Some(err)
                }
            }
            None
        },
        Stmnt::Line { expr } => dec_analy_expr(expr, scope_type, type_map),
        Stmnt::Conditional { conditional } => dec_analy_cond(conditional, scope_type, type_map),
        Stmnt::While { condition: _c, statement } => { dec_analy_stmnt(*statement, Rc::new(RefCell::new(scope_type.borrow_mut().bump())), type_map) }
        Stmnt::For { variable: _v, iterable: _i, statement } => { dec_analy_stmnt(*statement, Rc::new(RefCell::new(scope_type.borrow_mut().bump())), type_map) }
        Stmnt::InfLoop { statement } => dec_analy_stmnt(*statement, Rc::new(RefCell::new(scope_type.borrow_mut().bump())), type_map),
        Stmnt::Try { statement } => dec_analy_stmnt(*statement, Rc::new(RefCell::new(scope_type.borrow_mut().bump())), type_map),
        Stmnt::Except { catch: _c, statement } => dec_analy_stmnt(*statement, Rc::new(RefCell::new(scope_type.borrow_mut().bump())), type_map),
        Stmnt::Return { statement } => dec_analy_stmnt(*statement, scope_type, type_map)
    }
}

fn dec_analy_expr(expr: Expression, scope_type: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<IdentStore, ScopeSet>) -> Option<SyntaxError> {
    match expr.data {
        Expr::TypedAssignment { idents: ids, expr } => {
            for id in ids {
                match dec_analy_ident(id, scope_type.clone(), type_map) {
                    None => {}
                    Some(err) => return Some(err)
                }
            } None
        }
        _ => None
    }
}

fn dec_analy_cond(cond: Conditional, scope_value: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<IdentStore, ScopeSet>) -> Option<SyntaxError> {
    let statement = match cond {
        Conditional::If { condition: _c, statement } => statement,
        Conditional::ElseIf { condition: _c, statement } => statement,
        Conditional::Else { statement } => statement,
    };
    let bump = Rc::new(RefCell::new(scope_value.borrow_mut().bump()));
    dec_analy_stmnt(*statement, bump, type_map)
}

fn dec_analy_ident(expr: Expression, scope_value: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<IdentStore, ScopeSet>) -> Option<SyntaxError> {
    let mut arg_types = vec![];
    if let Expr::TypedVariable { typing: t, variable } = expr.data {
        if let LeBlancType::Derived(DerivedType::TypedList(inner_type)) = t {
            arg_types.push(type_value(inner_type.str()));
        }
        let arg_scope = ScopeValue {
            scope: scope_value.borrow().get_scope_type(),
            arg_types,
            types: vec![t],
            id: vid_pp()
        };
        match add_to_type_map(type_map, IdentStore::Variable(variable), arg_scope) {
            Ok(_) => {}, Err(err) => return Some(err)
        }
    }
    None
}
