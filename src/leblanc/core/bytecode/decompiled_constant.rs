use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::native_types::arch_type::leblanc_object_arch;
use crate::leblanc::core::native_types::boolean_type::leblanc_object_boolean;
use crate::leblanc::core::native_types::char_type::leblanc_object_char;
use crate::leblanc::core::native_types::double_type::leblanc_object_double;
use crate::leblanc::core::native_types::float_type::leblanc_object_float;
use crate::leblanc::core::native_types::int128_type::leblanc_object_int128;
use crate::leblanc::core::native_types::int64_type::leblanc_object_int64;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::short_type::leblanc_object_short;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::LeBlancType;

#[derive(Debug)]
pub struct DecompiledConstant {
    constant_data: Hexadecimal,
    constant_type: LeBlancType
}

impl DecompiledConstant {
    pub fn new(constant_data: Hexadecimal, constant_type: LeBlancType) -> DecompiledConstant {
        return DecompiledConstant {
            constant_data,
            constant_type
        }
    }

    pub fn to_leblanc_object(self) -> LeBlancObject {
        println!("{:#?}", self);
        return match self.constant_type {
            LeBlancType::Char => leblanc_object_char(char::from_hex(&self.constant_data)),
            LeBlancType::Short => leblanc_object_short(i16::from_hex(&self.constant_data)),
            LeBlancType::Int => leblanc_object_int(i32::from_hex(&self.constant_data)),
            LeBlancType::Int64 => leblanc_object_int64(i64::from_hex(&self.constant_data)),
            LeBlancType::Int128 => leblanc_object_int128(i128::from_hex(&self.constant_data)),
            LeBlancType::Arch => leblanc_object_arch(isize::from_hex(&self.constant_data)),
            LeBlancType::Float => leblanc_object_float(f32::from_hex(&self.constant_data)),
            LeBlancType::Double => leblanc_object_double(f64::from_hex(&self.constant_data)),
            LeBlancType::Boolean => leblanc_object_boolean(bool::from_hex(&self.constant_data)),
            LeBlancType::String => leblanc_object_string(String::from_hex(&self.constant_data)),
            _ => LeBlancObject::error()
        }
    }
}