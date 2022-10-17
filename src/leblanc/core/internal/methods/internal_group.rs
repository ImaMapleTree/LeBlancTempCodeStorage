use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast, Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::group_type::LeblancGroup;
use crate::leblanc::rustblanc::types::LBObject;

pub fn _internal_group_apply_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.read();
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.apply(_arguments[0].clone(), _arguments[1..].to_vec());
    true.create_mutex()
}

pub fn _internal_group_pipe_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.read();
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.pipe(_arguments);
    true.create_mutex()
}

pub fn _internal_group_pipe_async_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.read();
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.pipe_async(_arguments);
    true.create_mutex()
}