use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum LBOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
    Or,
    And,
    Not,
    Assign,
    AssignEach,
    Inverse,
    NotEquals,
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    LShift,
    RShift,
    Slice,
    Match,
    QuickList,
    Increment,
    Groupment,
    Index,
    Cast,
    NULL
}

pub fn is_operator(value: &str) -> bool {
    operator_type(value) != LBOperator::NULL
}

pub fn operator_type(value: &str) -> LBOperator {
    match value {
        "+" => LBOperator::Plus,
        "-" => LBOperator::Minus,
        "*" => LBOperator::Multiply,
        "/" => LBOperator::Divide,
        "**" => LBOperator::Power,
        "%" => LBOperator::Modulo,
        "or" => LBOperator::Or,
        "and" => LBOperator::And,
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
        ":" => LBOperator::Slice,
        "to" => LBOperator::QuickList,
        "in" => LBOperator::AssignEach,
        "by" => LBOperator::Increment,
        "as" => LBOperator::Cast,
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
            LBOperator::Or => "or",
            LBOperator::And => "and",
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
            LBOperator::Slice => ":",
            LBOperator::Match => "=>",
            LBOperator::AssignEach => "in",
            LBOperator::Increment => "by",
            LBOperator::QuickList => "to",
            LBOperator::Cast => "as",
            LBOperator::Index => "[",
            LBOperator::NULL => "null",
            LBOperator::Groupment => "->"
        };
        write!(f, "{}", s)
    }
}