use crate::leblanc::compiler3::leblanc_type::LBType;
use crate::leblanc::compiler::parser::ast::{Expression, Location};
use crate::leblanc::rustblanc::copystring::CopyString;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FunctionArg {
    pub(crate) name: CopyString,
    pub(crate) typing: LBType,
    pub(crate) location: Location
}

impl From<Expression> for FunctionArg {
    fn from(expr: Expression) -> Self {
        let (typing, ident) = expr.data.into_typed_variable().unwrap();
        FunctionArg { name: CopyString::from(ident.resolve()), typing: LBType::from(typing), location: expr.location }
    }
}