use crate::leblanc::compiler::generator::generator_types::GeneratedFuncHeader;
use crate::leblanc::compiler::parser::ast::{BinaryOperator, Comparator, Constant, Expr, Expression, Ident, Statement};
use crate::leblanc::core::native_types::LeBlancType;

pub struct TypedVariable {
    pub typing: LeBlancType,
    pub variable: Ident
}

impl TypedVariable {
    pub fn new(typing: LeBlancType, variable: Ident) -> TypedVariable {
        TypedVariable {
            typing,
            variable
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Property {
    pub typing: LeBlancType,
    pub ident: String,
    pub value: Option<Expression>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Function {
    pub header: GeneratedFuncHeader,
    pub body: Statement,
    pub tags: Vec<String>
}

pub struct RangeExpression {
    pub start: Expression,
    pub bound: Expression,
    pub step: Expression
}

pub struct StaticMethodCall {
    pub method_name: Expression,
    pub args: Vec<Expression>
}

pub struct ClassMethodCall {
    pub class: Expression,
    pub method_name: Ident,
    pub args: Vec<Expression>
}

pub struct ListIndex {
    pub list: Expression,
    pub slice: Expr
}

pub struct Slice {
    pub left: Expression,
    pub right: Expression
}

pub struct SeriesIndex {
    pub indices: Vec<Expression>
}

pub struct Equality {
    pub left: Expression,
    pub comparator: Comparator,
    pub right: Expression
}

pub struct List {
    pub items: Expression
}

pub struct ArithPlusMinusOperation {
    pub left: Expression,
    pub op: BinaryOperator,
    pub right: Expression
}

pub struct ArithMulDivModOperation {
    pub left: Expression,
    pub op: BinaryOperator,
    pub right: Expression
}

pub struct ExponentialOperation {
    pub left: Expression,
    pub op: BinaryOperator,
    pub right: Expression
}

pub struct UnaryOperator {
    pub term: Expression,
    pub op: BinaryOperator
}

pub struct IncrementDecrementOperation {
    pub term: Expression,
    pub op: UnaryOperator,
    pub postfix: bool
}

pub struct ListAssignment {
    pub list: Expression,
    pub expr: Expression
}

pub struct TypedAssignment {
    pub idents: Vec<Expression>,
    pub expr: Option<Expression>
}

pub struct NormalAssignment {
    pub idents: Vec<Ident>,
    pub expr: Expression
}

pub struct GroupAssignment {
    pub assignee: Expression,
    pub group: Expression
}

pub struct BlockLambda {
    pub variables: Vec<Ident>,
    pub expr: Statement
}

pub struct ExprLambda {
    pub variables: Vec<Ident>,
    pub expr: Expression
}

pub struct ExceptCatch {
    pub errors: Vec<LeBlancType>,
    pub variable: String
}

pub struct ExprIdent {
    pub ident: Ident
}

pub struct ExprConst {
    pub constant: Constant
}