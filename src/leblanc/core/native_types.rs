use std::fmt::{Display, Formatter};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType::*;

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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Ord, PartialOrd, Hash)]
pub enum LeBlancType {
    Class(u32), // User defined class with ID
    Flex,
    RealFlex, // internal implementation of "flex"
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
    RealDynamic // internal implementation of "dynamic
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

    pub fn as_str_real(&self) -> std::string::String {
        return match self {
            Class(v) => "class.".to_owned() + &v.to_string(),
            _ => self.as_str().to_string()
        }
    }

    pub fn as_str(&self) ->&str {
        return match self {
            Flex => "flex",
            RealFlex => "flex",
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
            RealDynamic => "dynamic"
        }
    }
}

impl Display for LeBlancType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str_real())
    }
}