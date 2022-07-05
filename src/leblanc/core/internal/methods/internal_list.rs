use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::internal::internal_list_iterator::LeblancVecIterator;
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;


pub fn _internal_list_append_(_self: Arc<Strawberry<LeBlancObject>>, arguments: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.lock();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    list.internal_vec.push(arguments[0].clone());
    LeBlancObject::unsafe_null()
}

pub fn _internal_list_iterate_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.lock();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_iterator(Box::new(LeblancVecIterator::new(list.internal_vec.clone()))).to_mutex()
}
pub fn _internal_list_length_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.lock();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_int(list.internal_vec.len() as i32).to_mutex()
}

