use serde_value::{to_value, Value};
use crate::leblanc::core::leblanc_object::{LeBlancObject, Reflect};
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _internal_add_number_(_self: &LeBlancObject, arguments: &[LeBlancObject]) -> LeBlancObject {
    let n1: i64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: i64 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    return result.create();
}

pub fn _internal_add_double_(_self: &LeBlancObject, arguments: &[LeBlancObject]) -> LeBlancObject {
    let n1: f64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f64 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    return result.create();
}

pub fn _internal_add_float_(_self: &LeBlancObject, arguments: &[LeBlancObject]) -> LeBlancObject {
    let n1: f32 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f32 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    return result.create();
}