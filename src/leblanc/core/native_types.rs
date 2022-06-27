use core::str::FromStr;

use std::fmt::{Display, Formatter};

use crate::leblanc::core::native_types::derived::DerivedType;

use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::LeBlancType::{Arch, Block, Boolean, Char, Class, Derived, Double, Dynamic, Exception, Flex, Float, Function, Int, Int128, Int64, Module, Null, SelfType, Short};

pub mod NULL;
pub mod string_type;
pub mod base_type;
pub mod block_type;
pub mod int_type;
pub mod int64_type;
pub mod arch_type;
pub mod int128_type;
pub mod boolean_type;
pub mod double_type;
pub mod float_type;
pub mod short_type;
pub mod class_type;
pub mod char_type;
pub mod attributes;
pub mod derived;

static VARIANTS: [&str; 18] = ["flex", "Self", "char", "short", "int", "int64", "in128", "arch", "float", "double", "boolean", "string", "block", "function", "module", "class", "dynamic", "null"];

#[derive(Eq, Clone, Copy, Debug, Ord, PartialOrd, Hash)]
pub enum LeBlancType {
    Class(u32), // User defined class with ID
    Flex,
    SelfType, // internal implementation of "flex"
    Char,
    Short,
    Int,
    Int64,
    Int128,
    Arch,
    Float, //"double32" -- internally f32
    Double, // internally f64
    Boolean,
    String,
    Block,
    Function,
    Module,
    Dynamic,
    Exception, // internal implementation of "dynamic
    Derived(DerivedType),
    Null
}

pub fn is_native_type(string: &str) -> bool { type_value(string) != Class(0) }

pub fn type_value(string: &str) -> LeBlancType {
    return match string {
        "flex" => Flex,
        "char" => Char,
        "short" => Short,
        "int" => Int,
        "int64" => Int64,
        "int128" => Int128,
        "arch" => Arch,
        "float" => Float,
        "double32" => Float,
        "double" => Double,
        "boolean" => Boolean,
        "string" => LeBlancType::String,
        "block" => Block,
        "function" => Function,
        "module" => Module,
        "dynamic" => Dynamic,
        "exception" => Exception,
        "Self" => SelfType,
        "null" => Null,
        Other => {
            if Other.starts_with("class.") {
                let class_value = Other[6..].parse::<u32>().unwrap();
                Class(class_value)
            } else {
                Class(0)
            }
        }
    }
}

impl LeBlancType {
    pub fn is_numeric(&self) -> bool {
        return match self {
            Short => true,
            Int => true,
            Int64 => true,
            Int128 => true,
            Arch => true,
            Float => true,
            Double => true,
            Boolean => true,
            _ => false
        }
    }

    pub fn is_native(&self) -> bool {
        return match self {
            Class(value) => *value == 0,
            Derived(_) => false,
            _ => true
        }
    }

    pub fn as_str_real(&self) -> String {
        return match self {
            Class(v) => "class.".to_string() + &v.to_string(),
            _ => self.as_str().to_string()
        }
    }

    pub fn as_str(&self) ->&str {
        return match self {
            Flex => "flex",
            SelfType => "Self",
            Char => "char",
            Short => "short",
            Int => "int",
            Int64 => "int64",
            Int128 => "int128",
            Arch => "arch",
            Float => "float",
            Double => "double",
            Boolean => "boolean",
            LeBlancType::String => "string",
            Block => "block",
            Function => "function",
            Module => "module",
            Class(v) => {
                if *v == 0 {
                    "Undefined"
                } else {
                    "class"
                }
            }
            Dynamic => "dynamic",
            Exception => "exception",
            Derived(Derive) => {
                match Derive {
                    DerivedType::List => "list",
                    DerivedType::Iterator => "iterator"
                }
            }
            Null => "null"
        }
    }

    pub fn enum_id(&self) -> u32 {
        VARIANTS.iter().position(|&s| s == self.as_str()).unwrap() as u32
    }

    pub fn from_enum_id(id: u16) -> LeBlancType {
        type_value(VARIANTS[id as usize])
    }

    pub fn transform(&self, string: std::string::String) -> Hexadecimal {
        return match self {
            Char => string.chars().next().unwrap().to_hex(128),
            Short => i16::from_str(string.as_str()).unwrap().to_hex(128),
            Int => i32::from_str(string.as_str()).unwrap().to_hex(128),
            Int64 => i64::from_str(string.as_str()).unwrap().to_hex(128),
            Int128 => i128::from_str(string.as_str()).unwrap().to_hex(128),
            Arch => usize::from_str(string.as_str()).unwrap().to_hex(128),
            Float => f32::from_str(string.as_str()).unwrap().to_hex(128),
            Double => f64::from_str(string.as_str()).unwrap().to_hex(128),
            Boolean => bool::from_str(string.as_str()).unwrap().to_hex(128),
            _String => string[1..string.len()-1].to_string().to_hex(128),
            _ => string.to_hex(128)
        }
    }
}

impl Display for LeBlancType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str_real())
    }
}

impl PartialEq for LeBlancType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Flex => true,
            Dynamic => true,
            _ => self.as_str_real() == other.as_str_real()
        }
    }
}