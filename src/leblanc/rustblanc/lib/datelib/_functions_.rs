use alloc::rc::Rc;
use std::cell::RefCell;
use std::ops::Div;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;
use num::ToPrimitive;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _epoch_(_self: Arc<Strawberry<LeBlancObject>>, _args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    Local::now().timestamp_nanos().to_f64().unwrap().div(1e9).create_mutex()
}

pub fn _epoch_seconds_(_self: Arc<Strawberry<LeBlancObject>>, _args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    Local::now().timestamp().create_mutex()
}

pub fn _sleep_(_self: Arc<Strawberry<LeBlancObject>>, _args: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    thread::sleep(Duration::from_secs(_args[0].lock().data.as_i128() as u64));
    LeBlancObject::unsafe_null()
}