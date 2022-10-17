use core::borrow::BorrowMut;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc};
use crate::leblanc::core::internal::internal_list_iterator::LeblancVecIterator;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::rustblanc::types::LBObject;


pub fn _internal_list_append_(_self: Arc<Strawberry<LeBlancObject>>, arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    //<LeBlancObjectData as RustDataCast<LeblancList>>::mut_data(&mut _self.underlying_pointer().data).unwrap().internal_vec.push(arguments[0].clone());
    <LeBlancObjectData as RustDataCast<LeblancList>>::mut_data(&mut _self.write().data).unwrap().internal_vec.push(arguments[0].clone());
    LeBlancObject::unsafe_null()
}

pub fn _internal_list_iterate_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.write();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_iterator(Box::new(LeblancVecIterator::new(list.internal_vec.clone()))).to_mutex()
}
pub fn _internal_list_length_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.write();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_int(list.internal_vec.len() as i32).to_mutex()
}

