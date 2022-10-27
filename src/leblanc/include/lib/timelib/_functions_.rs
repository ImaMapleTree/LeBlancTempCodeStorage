
use std::ops::Div;


use std::thread;
use std::time::Duration;
use chrono::Local;
use num::ToPrimitive;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::native_types::base_type::ToLeblanc;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};

pub fn _epoch_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    Local::now().timestamp_nanos().to_f64().unwrap().div(1e9).create_mutex()
}

pub fn _epoch_seconds_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    Local::now().timestamp().create_mutex()
}

pub fn _sleep_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    thread::sleep(Duration::from_secs(_args[0].data.as_i128() as u64));
    LeBlancObject::unsafe_null()
}