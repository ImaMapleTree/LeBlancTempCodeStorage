use alloc::rc::Rc;
use std::ops::Div;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use chrono::Local;
use num::ToPrimitive;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::LBObject;

pub fn _epoch_(_self: LBObject, _args: Vec<LBObject>) -> LBObject {
    Local::now().timestamp_nanos().to_f64().unwrap().div(1e9).create_mutex()
}

pub fn _epoch_seconds_(_self: LBObject, _args: Vec<LBObject>) -> LBObject {
    Local::now().timestamp().create_mutex()
}

pub fn _sleep_(_self: LBObject, _args: Vec<LBObject>) -> LBObject {
    thread::sleep(Duration::from_secs(_args[0].data.as_i128() as u64));
    LeBlancObject::unsafe_null()
}