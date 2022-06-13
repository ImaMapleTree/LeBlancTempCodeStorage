use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum LBOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
    Not,
    Assign,
    Inverse,
    NotEquals,
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    LShift,
    RShift,
    Match,
    Cast,
    Attribute,
    NULL
}

pub fn is_operator(value: &str) -> bool {
    operator_type(value) != LBOperator::NULL
}

pub fn operator_type(value: &str) -> LBOperator {
    return match value {
        "+" => LBOperator::Plus,
        "-" => LBOperator::Minus,
        "*" => LBOperator::Multiply,
        "/" => LBOperator::Divide,
        "**" => LBOperator::Power,
        "%" => LBOperator::Modulo,
        "!" => LBOperator::Not,
        "~" => LBOperator::Inverse,
        "=" => LBOperator::Assign,
        "==" => LBOperator::Equals,
        "!=" => LBOperator::NotEquals,
        ">" => LBOperator::GreaterThan,
        "<" => LBOperator::LessThan,
        ">=" => LBOperator::GreaterThanOrEqual,
        "<=" => LBOperator::LessThanOrEqual,
        "<<" => LBOperator::LShift,
        ">>" => LBOperator::RShift,
        "=>" => LBOperator::Match,
        "." => LBOperator::Attribute,
        "to" => LBOperator::Cast,
        _ => LBOperator::NULL,
    }
}

impl Display for LBOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LBOperator::Plus => "+",
            LBOperator::Minus => "-",
            LBOperator::Multiply => "*",
            LBOperator::Divide => "/",
            LBOperator::Power => "**",
            LBOperator::Modulo => "%",
            LBOperator::Not => "!",
            LBOperator::Assign => "=",
            LBOperator::Inverse => "~",
            LBOperator::NotEquals => "!=",
            LBOperator::Equals => "==",
            LBOperator::GreaterThan => ">",
            LBOperator::LessThan => "<",
            LBOperator::GreaterThanOrEqual => ">=",
            LBOperator::LessThanOrEqual => "<=",
            LBOperator::LShift => "<<",
            LBOperator::RShift => ">>",
            LBOperator::Match => "=>",
            LBOperator::Cast => "to",
            LBOperator::Attribute => ".",
            LBOperator::NULL => "null"
        };
        write!(f, "{}", s)
    }
}