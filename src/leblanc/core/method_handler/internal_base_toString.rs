use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use crate::LeBlancType;

pub fn INTERNAL_TO_STRING(_self: &LeBlancObject, args: &[LeBlancObject]) -> LeBlancObject {
    return leblanc_object_string(_self.data.to_string());
}