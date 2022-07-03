

use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _internal_add_number_(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let n1: i128 = _self.lock().unwrap().data.as_i128();
    let n2: i128  = arguments[0].lock().unwrap().data.as_i128();
    (n1 + n2).create_mutex()
}

pub fn _internal_add_double_(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let n1: f64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f64 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    result.create_mutex()
}

pub fn _internal_add_float_(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let n1: f32 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: f32 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};

    let result = n1 + n2;

    result.create_mutex()
}

pub fn _internal_inplace_add_(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let n1: i64 = unsafe {*(_self.reflect().downcast_ref_unchecked())};
    let n2: i64 = unsafe {*(arguments[0].reflect().downcast_ref_unchecked())};
    println!("n1: {} | n2: {}", n1, n2);

    _self.lock().unwrap().data = LeBlancObjectData::Int64(n1 + n2);
    arguments[0].clone()
}