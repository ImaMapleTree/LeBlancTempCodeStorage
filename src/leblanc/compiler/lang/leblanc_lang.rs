use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{BraceClosed, BraceOpen, BracketClosed, BracketOpen, Comma, DNE, ParenthesisClosed, ParenthesisOpen, Semicolon};
use crate::leblanc::compiler::symbols::Symbol;
use crate::leblanc::core::native_types::LeBlancType::*;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_lang::Specials::{BlockCommentCloser, BlockCommentOpener, InlineComment, TagCloser, TagOpener};
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator;
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum CompileVocab {
    CONSTANT(LeBlancType),
    VARIABLE(LeBlancType),
    FUNCTION,
    OPERATOR(LBOperator),
    SPECIAL(Specials),
    KEYWORD(LBKeyword),
    MODULE(u64),
    BOUNDARY(BoundaryType),
    CONSTRUCTOR(LeBlancType),
    EXTENSION(LeBlancType),
    CLASS(LeBlancType),
    TYPE(LeBlancType),
    UNKNOWN(LeBlancType)
}

impl Symbol {
    pub fn is_boundary(&self) -> bool { boundary_value(self.char()) != DNE }
}

pub enum QuotationTypes {
    Single,
    Double
}

#[derive(PartialEq, Eq, Copy, Hash, Clone, Debug)]
pub enum BoundaryType {
    BracketOpen,
    BracketClosed,
    BraceOpen,
    BraceClosed,
    ParenthesisOpen,
    ParenthesisClosed,
    Semicolon,
    Comma,
    DNE
}

pub fn boundary_value(ch: &char) -> BoundaryType {
    return match ch {
        '[' => BracketOpen,
        ']' => BracketClosed,
        '{' => BraceOpen,
        '}' => BraceClosed,
        '(' => ParenthesisOpen,
        ')' => ParenthesisClosed,
        ',' => Comma,
        ';' =>  Semicolon,
        _ => DNE
    }
}

impl CompileVocab {
    pub fn extract_native_type(&self) -> &LeBlancType {
        return match self {
            CompileVocab::CONSTANT(native_type) => native_type,
            CompileVocab::VARIABLE(native_type) => native_type,
            CompileVocab::EXTENSION(native_type) => native_type,
            CompileVocab::CONSTRUCTOR(native_type) => native_type,
            CompileVocab::TYPE(native_type) => native_type,
            CompileVocab::UNKNOWN(native_type) => native_type,
            _ => self.extract_native_type(),
        }
    }

    pub fn matches(&self, pat: &str) -> bool {
        return match self {
            CompileVocab::CONSTANT(_) => pat.to_lowercase() == "constant",
            CompileVocab::VARIABLE(_) => pat.to_lowercase() == "variable",
            CompileVocab::FUNCTION => pat.to_lowercase() == "function",
            CompileVocab::OPERATOR(_) => pat.to_lowercase() == "operator",
            CompileVocab::SPECIAL(_) => pat.to_lowercase() == "special",
            CompileVocab::KEYWORD(_) => pat.to_lowercase() == "keyword",
            CompileVocab::MODULE(_) => pat.to_lowercase() == "module",
            CompileVocab::BOUNDARY(_) => pat.to_lowercase() == "boundary",
            CompileVocab::CONSTRUCTOR(_) => pat.to_lowercase() == "constructor",
            CompileVocab::EXTENSION(_) => pat.to_lowercase() == "extension",
            CompileVocab::TYPE(_) => pat.to_lowercase() == "type",
            CompileVocab::UNKNOWN(_) => pat.to_lowercase() == "unknown",
            CompileVocab::CLASS(_) => pat.to_lowercase() == "class"
        }
    }

    pub fn priority(&self) -> u16 {
        return match self {
            CompileVocab::KEYWORD(_) => 0,
            CompileVocab::OPERATOR(op) => {
                if *op == LBOperator::Assign {
                    0
                } else {
                    10
                }
            },
            CompileVocab::MODULE(_) => 20,
            CompileVocab::CLASS(_) => 30,
            CompileVocab::EXTENSION(_) => 40,
            CompileVocab::FUNCTION => 50,
            CompileVocab::CONSTRUCTOR(_) => 60,
            CompileVocab::TYPE(_) => 70,
            CompileVocab::VARIABLE(_) => 80,
            CompileVocab::CONSTANT(_) => 100,
            CompileVocab::BOUNDARY(_) => 110,
            CompileVocab::SPECIAL(_) => 120,
            CompileVocab::UNKNOWN(_) => 130,
        }
    }
}

impl Display for CompileVocab {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CompileVocab::CONSTANT(inner) => "constant.".to_string() + &inner.to_string(),
            CompileVocab::VARIABLE(inner) => "variable.".to_string() + &inner.to_string(),
            CompileVocab::FUNCTION => "function".to_string(),
            CompileVocab::OPERATOR(inner) => "operator.".to_string() + &inner.to_string(),
            CompileVocab::SPECIAL(inner) => "special.".to_string() + &inner.to_string(),
            CompileVocab::KEYWORD(inner) => "keyword.".to_string() + &inner.to_string(),
            CompileVocab::MODULE(inner) => "module.".to_string() + &inner.to_string(),
            CompileVocab::BOUNDARY(inner) => "boundary.".to_string() + &inner.to_string(),
            CompileVocab::CONSTRUCTOR(inner) => "constructor.".to_string() + &inner.to_string(),
            CompileVocab::EXTENSION(inner) => "extension.".to_string() + &inner.to_string(),
            CompileVocab::CLASS(inner) => "class.".to_string() + &inner.to_string(),
            CompileVocab::TYPE(inner) => "type.".to_string() + &inner.to_string(),
            CompileVocab::UNKNOWN(inner) => "unknown.".to_string() + &inner.to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Specials {
    InlineComment,
    BlockCommentOpener,
    BlockCommentCloser,
    TagOpener,
    TagCloser,
    DNE
}

pub fn is_special(string: &str) -> bool { special_value(string) != Specials::DNE }

pub fn special_value(string: &str) -> Specials {
    return match string {
        "//" => InlineComment,
        "/*" => BlockCommentOpener,
        "*/" => BlockCommentCloser,
        "<|" => TagOpener,
        "|>" => TagCloser,
        _ => Specials::DNE
    }
}

impl Display for Specials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            InlineComment => "//",
            BlockCommentOpener => "/*",
            BlockCommentCloser => "*/",
            TagOpener => "<|",
            TagCloser => "|>",
            Specials::DNE => "dne"
        };
        write!(f, "{}", s)
    }
}

impl Display for BoundaryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s= match self {
            BracketOpen => "[",
            BracketClosed => "]",
            BraceOpen => "{",
            BraceClosed => "}",
            ParenthesisOpen => "(",
            ParenthesisClosed => ")",
            Semicolon => ";",
            Comma => ",",
            DNE => "dne"
        };
        write!(f, "{}", s)
    }
}