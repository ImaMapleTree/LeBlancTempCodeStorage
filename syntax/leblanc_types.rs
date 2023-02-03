use crate::leblanc::core::native_types::LeBlancType::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
pub enum LeBlancType {
    Flex(&'static LeBlancType),
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
    Class(i64), // User defined class with ID
    Dynamic(&'static LeBlancType),
    RealDynamic // internal implementation of "dynamic
}

pub fn is_native_type(string: &str) -> bool { type_value(string) != Class(0) }

pub fn type_value(string: &str) -> LeBlancType {
    return match string {
        "flex" => Flex(&Class(0)),
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
        "dynamic" => Dynamic(&Class(0)),
        _ => Class(0)
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

    pub fn as_str(&self) ->&str {
        return match self {
            Flex(_) => "flex",
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
            Dynamic(_) => "dynamic",
            RealDynamic => "dynamic"
        }
    }
}