use std::fmt::{Display, Formatter};
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{BraceClosed, BraceOpen, BracketClosed, BracketOpen, Comma, DNE, ParenthesisClosed, ParenthesisOpen, Semicolon, VerticalLine};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_lang::Specials::{BlockCommentCloser, BlockCommentOpener, Dot, InlineComment, LambdaMarker, RangeMarker, SliceStart, StackAppend, TagCloser, TagOpener};
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum CompileVocab {
    CONSTANT(LeBlancType),
    VARIABLE(LeBlancType),
    FUNCTION(FunctionType),
    OPERATOR(LBOperator),
    SPECIAL(Specials, u16),
    KEYWORD(LBKeyword),
    MODULE(u64),
    BOUNDARY(BoundaryType),
    CONSTRUCTOR(LeBlancType),
    EXTENSION(ExtensionType),
    CLASS(LeBlancType),
    TYPE(LeBlancType),
    UNKNOWN(LeBlancType)
}

pub enum QuotationTypes {
    Single,
    Double
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub enum FunctionType {
    Header,
    Call,
    Reference,
    ReferenceCall,
    DNE
}

pub fn function_type_value(string: &str) -> FunctionType {
    match string {
        "header" => FunctionType::Header,
        "call" => FunctionType::Call,
        "reference" => FunctionType::Reference,
        _=> FunctionType::DNE
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            FunctionType::Header => "header",
            FunctionType::Call => "call",
            _ => "DNE"
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ExtensionType {
    ExtensionTypeImport(u32),
    ExtensionTypeExport(u32),
    ExtensionTypeParam(LeBlancType),
}

impl Display for ExtensionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
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
    VerticalLine,
    Comma,
    DNE
}

pub fn boundary_value(ch: &char) -> BoundaryType {
    match ch {
        '[' => BracketOpen,
        ']' => BracketClosed,
        '{' => BraceOpen,
        '}' => BraceClosed,
        '(' => ParenthesisOpen,
        ')' => ParenthesisClosed,
        '|' => VerticalLine,
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
            CompileVocab::CONSTRUCTOR(native_type) => native_type,
            CompileVocab::TYPE(native_type) => native_type,
            CompileVocab::UNKNOWN(native_type) => native_type,
            _ => self.extract_native_type(),
        }
    }

    pub fn matches(&self, pat: &str) -> bool {
        match self {
            CompileVocab::CONSTANT(_) => pat.to_lowercase() == "constant",
            CompileVocab::VARIABLE(_) => pat.to_lowercase() == "variable",
            CompileVocab::FUNCTION(_) => pat.to_lowercase() == "function",
            CompileVocab::OPERATOR(_) => pat.to_lowercase() == "operator",
            CompileVocab::SPECIAL(..) => pat.to_lowercase() == "special",
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

    pub fn stores_native_type(&self) -> bool {
        !matches!(self, CompileVocab::FUNCTION(_) | CompileVocab::OPERATOR(_) | CompileVocab::SPECIAL(..) | CompileVocab::KEYWORD(_) | CompileVocab::MODULE(_) | CompileVocab::BOUNDARY(_))
    }

    pub fn priority(&self) -> u16 {
        match self {
            CompileVocab::KEYWORD(_) => 1,
            CompileVocab::OPERATOR(op) => {
                match *op {
                    LBOperator::Assign => 1,
                    LBOperator::AssignEach => 5,
                    LBOperator::Increment => 9,
                    LBOperator::QuickList => 8,
                    LBOperator::Or | LBOperator::And => 5,
                    LBOperator::Index => 11,
                    _ => 10
                }
            },
            CompileVocab::MODULE(_) => 20,
            CompileVocab::CLASS(_) => 30,
            CompileVocab::EXTENSION(_) => 40,
            CompileVocab::FUNCTION(_) => 50,
            CompileVocab::CONSTRUCTOR(_) => 60,
            CompileVocab::TYPE(_) => 70,
            CompileVocab::VARIABLE(_) => 100,
            CompileVocab::CONSTANT(_) => 100,
            CompileVocab::BOUNDARY(boundary) => {
                match *boundary {
                    Semicolon => 0,
                    _ => 110
                }
            },
            CompileVocab::SPECIAL(_, priority) => *priority,
            CompileVocab::UNKNOWN(_) => 130,
        }
    }
}

impl Display for CompileVocab {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CompileVocab::CONSTANT(inner) => "constant.".to_string() + &inner.to_string(),
            CompileVocab::VARIABLE(inner) => "variable.".to_string() + &inner.to_string(),
            CompileVocab::FUNCTION(inner) => "function.".to_string() + &inner.to_string(),
            CompileVocab::OPERATOR(inner) => "operator.".to_string() + &inner.to_string(),
            CompileVocab::SPECIAL(inner, ..) => "special.".to_string() + &inner.to_string(),
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
    RangeMarker,
    Dot,
    StackAppend,
    SliceStart,
    LambdaMarker,
    DNE
}

pub fn is_special(string: &str) -> bool { special_value(string) != Specials::DNE }

pub fn special_value(string: &str) -> Specials {
    match string {
        "//" => InlineComment,
        "/*" => BlockCommentOpener,
        "*/" => BlockCommentCloser,
        "<|" => TagOpener,
        "|>" => TagCloser,
        "." => Dot,
        "->" => StackAppend,
        "|" => LambdaMarker,
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
            Dot => ".",
            RangeMarker => "to",
            StackAppend => "->",
            SliceStart => "[",
            LambdaMarker => "|",
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
            VerticalLine => "|",
            DNE => "dne"
        };
        write!(f, "{}", s)
    }
}