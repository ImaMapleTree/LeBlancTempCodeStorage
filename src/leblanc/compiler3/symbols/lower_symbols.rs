use crate::leblanc::compiler3::symbols::symbol_table::{STCode, STIdent};
use crate::leblanc::compiler::parser::ast::{BinaryOperator, Const};

pub enum STSymbol {
    IfElseBlock(Vec<Vec<STCode>>),
    Assignment(STIdent, STCode),
    MethodCall(Box<STMethodRef>, Vec<STSymbol>),
    BinaryOperation(STOperand, STOperator, STOperand),
    While(Box<STSymbol>, Vec<STCode>),
    ForLoop(STIdent, Box<STSymbol>, Vec<STCode>),
    Loop(Vec<STCode>),
    Return(Vec<STCode>),

}

pub enum STMethodRef {
    Named(STIdent),
    Other(STSymbol),
}

pub enum STOperand {
    Constant(STConstant),
    Variable(STIdent)
}

pub struct STConstant(Const);

pub struct STOperator(BinaryOperator);