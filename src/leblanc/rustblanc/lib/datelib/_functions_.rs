use alloc::rc::Rc;
use std::cell::RefCell;
use std::ops::Div;
use chrono::Local;
use num::ToPrimitive;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _epoch_(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    Local::now().timestamp_nanos().to_f64().unwrap().div(1e9).create_mutex()
}

pub fn _epoch_seconds_(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    Local::now().timestamp().create_mutex()
}