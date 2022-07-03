use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast, Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::group_type::LeblancGroup;

pub fn _internal_group_apply_(_self: Arc<Mutex<LeBlancObject>>, _arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let mut borrowed = _self.lock().unwrap();
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.apply(_arguments[0].clone(), &mut _arguments[1..]);
    true.create_mutex()
}

pub fn _internal_group_pipe_(_self: Arc<Mutex<LeBlancObject>>, _arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let mut borrowed = _self.lock().unwrap();
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.pipe(_arguments);
    true.create_mutex()
}

pub fn _internal_group_pipe_async_(_self: Arc<Mutex<LeBlancObject>>, _arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let mut borrowed = _self.lock().unwrap();
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.pipe_async(_arguments);
    true.create_mutex()
}