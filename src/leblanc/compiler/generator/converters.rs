use crate::leblanc::compiler::generator::generator_types::FunctionSignature;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Conditional, Expr, Expression, LConditional, Statement, Stmnt};
use crate::leblanc::compiler::parser::ast_structs::{Function, Property, TypedVariable};

pub fn expr_to_typed_var(expr: &Vec<Expression>) -> Vec<TypedVariable> {
    expr.iter().filter_map(|e|
        if let Expr::TypedVariable {typing, variable} = &e.data {
            Some(TypedVariable::new(*typing, variable.clone()))
        } else { None }
    ).collect()
}

pub fn stmnt_to_conditional(stmnt: &Vec<Statement>) -> Vec<LConditional> {
    stmnt.iter().filter_map(|e| {
        if let Stmnt::Conditional { conditional} = &e.data {
            Some(LConditional::new(e.location, conditional.clone()))
        } else { None }
    }).collect()
}

/*pub fn cmpt_to_property(cmpt: &Vec<Component>) -> Vec<Property> {
    cmpt.iter().filter_map(|c|
        if let Cmpnt::Property {typing, ident, value} = &c.data {
            Some(Property { typing: *typing, ident: ident.clone(), value: value.clone()})
        } else { None }
    ).collect()
}

pub fn cmpt_to_function(cmpt: &Vec<Component>) -> Vec<Function> {
    cmpt.iter().filter_map(|c|
        if let Cmpnt::Function {header, body, tags} = &c.data {
            Some(Function { header: FunctionSignature::from(&header.data), body: body.to_owned(),
            tags: tags.clone()})
        } else { None }
    ).collect()
}*/