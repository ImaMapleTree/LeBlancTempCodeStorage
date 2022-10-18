use core::borrow::BorrowMut;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc};
use crate::leblanc::core::internal::internal_list_iterator::LeblancVecIterator;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::LBObject;


pub fn _internal_list_append_(mut _self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    //<LeBlancObjectData as RustDataCast<LeblancList>>::mut_data(&mut _self.data).unwrap().internal_vec.push(arguments[0].clone());
    <LeBlancObjectData as RustDataCast<LeblancList>>::mut_data(&mut _self.data).unwrap().internal_vec.push(arguments[0].clone());
    LeBlancObject::unsafe_null()
}

pub fn _internal_list_iterate_(mut _self: LBObject, _arguments: Vec<LBObject>) -> LBObject {
    let mut borrowed = _self.clone();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_iterator(Box::new(LeblancVecIterator::new(list.internal_vec.clone())))
}
pub fn _internal_list_length_(_self: LBObject, _arguments: Vec<LBObject>) -> LBObject {
    let mut borrowed = _self;
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_int(list.internal_vec.len() as i32)
}

