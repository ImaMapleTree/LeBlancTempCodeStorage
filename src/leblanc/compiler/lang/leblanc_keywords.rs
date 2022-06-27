use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Copy, Hash, Clone, Debug)]
pub enum LBKeyword {
    Using,
    Return,
    Returns,
    Func,
    For,
    From,
    While,
    Global,
    Extension,
    ExtensionImport,
    None,
    Of,
    If,
    ElseIf,
    Else,
    SelfRT,// Reference and Type
    Class,
    Null
}

pub fn is_keyword(value: &str) -> bool { keyword_value(value) != LBKeyword::Null }

pub fn keyword_value(value: &str) -> LBKeyword {
    match value {
        "using" => LBKeyword::Using,
        "return" => LBKeyword::Return,
        "returns" => LBKeyword::Returns,
        "func" => LBKeyword::Func,
        "for" => LBKeyword::For,
        "from" => LBKeyword::From,
        "while" => LBKeyword::While,
        "global" => LBKeyword::Global,
        "Extension" => LBKeyword::Extension,
        "extension" => LBKeyword::ExtensionImport,
        "ext" => LBKeyword::ExtensionImport,
        "Class" => LBKeyword::Class,
        "of" => LBKeyword::Of,
        "Self" => LBKeyword::SelfRT,
        "if" => LBKeyword::If,
        "elif" => LBKeyword::ElseIf,
        "else" => LBKeyword::Else,
        _ => LBKeyword::Null
    }
}

impl Display for LBKeyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LBKeyword::Using => "using",
            LBKeyword::Return => "return",
            LBKeyword::Returns => "returns",
            LBKeyword::Func => "func",
            LBKeyword::From => "from",
            LBKeyword::For => "for",
            LBKeyword::While => "while",
            LBKeyword::Global => "global",
            LBKeyword::Extension => "Extension",
            LBKeyword::ExtensionImport => "ext",
            LBKeyword::None => "none",
            LBKeyword::Of => "of",
            LBKeyword::If => "if",
            LBKeyword::ElseIf => "elseif",
            LBKeyword::Else => "else",
            LBKeyword::SelfRT => "selfrt",
            LBKeyword::Class => "class",
            LBKeyword::Null => "null"
        };
        write!(f, "{}", s)
    }
}

