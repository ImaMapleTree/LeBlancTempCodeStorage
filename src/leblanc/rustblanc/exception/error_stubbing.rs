use std::fmt::{Display, Formatter};
use crate::leblanc::compiler::symbols::Symbol;
use crate::leblanc::compiler::identifier::typed_token::TypedToken;

#[derive(Clone, Debug)]
pub enum ErrorStub {
    ParseImbalancedQuotation(u32, u32),
    ImbalancedDelimiter(Symbol),
    MissingSemicolon(u32, u32),
    UndeclaredVariable(TypedToken),
    InvalidGlobalVariableDeclaration(TypedToken),
    FlexReassignment(TypedToken),
    IncompatibleType(TypedToken),
    VariableAlreadyDefined(TypedToken),
    InvalidSyntax(TypedToken),
}

impl Display for ErrorStub {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}