
use crate::leblanc::rustblanc::strawberry::{Either, Strawberry};
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _internal_add_number_(_self: Strawberry<LeBlancObject>, arguments: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    let n1: i128 = _self.loan().inquire_uncloned().either().data.as_i128();
    let n2: i128  = arguments[0].loan().inquire_uncloned().either().data.as_i128();
    return (n1 + n2).create_mutex();
}

pub fn _internal_add_double_(_self: Strawberry<LeBlancObject>, arguments: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    let n1: f64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f64 = unsafe {*(arguments[0].loan().inquire_uncloned().either().reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    return result.create_mutex();
}

pub fn _internal_add_float_(_self: Strawberry<LeBlancObject>, arguments: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    let n1: f32 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f32 = unsafe {*(arguments[0].loan().inquire_uncloned().either().reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    return result.create_mutex();
}

pub fn _internal_inplace_add_(_self: Strawberry<LeBlancObject>, arguments: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    let n1: i64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: i64 = unsafe {*(arguments[0].loan().inquire_uncloned().either().reflect().downcast_ref_unchecked())};
    println!("n1: {} | n2: {}", n1, n2);

    _self.loan().inquire().either().data = LeBlancObjectData::Int64(n1 + n2);
    return arguments[0].clone();
}