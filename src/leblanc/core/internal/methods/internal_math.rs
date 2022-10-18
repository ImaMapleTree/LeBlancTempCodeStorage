

use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::LBObject;


pub fn _internal_add_number_(_self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    let n1: i64 = _self.data.as_i64();
    let n2: i64  = arguments[0].data.as_i64();
    (n1 + n2).create_mutex()
}

pub fn _internal_add_double_(_self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    let n1: f64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f64 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    result.create_mutex()
}

pub fn _internal_add_float_(_self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    let n1: f32 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f32 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    result.create_mutex()
}

pub fn _internal_inplace_add_(mut _self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    let n1: i64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: i64 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};
    println!("n1: {} | n2: {}", n1, n2);

    _self.data = LeBlancObjectData::Int64(n1 + n2);
    arguments[0].clone()
}