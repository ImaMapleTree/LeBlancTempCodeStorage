


use crate::leblanc::core::internal::internal_list_iterator::LeblancVecIterator;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;


pub fn _internal_list_append_(mut _self: LBObject, arguments: LBObjArgs) -> LBObject {
    //<LeBlancObjectData as RustDataCast<LeblancList>>::mut_data(&mut _self.data).unwrap().internal_vec.push(arguments[0].clone());
    <LeBlancObjectData as RustDataCast<LeblancList>>::mut_data(&mut _self.data).unwrap().internal_vec.push(arguments[0].clone());
    LeBlancObject::unsafe_null()
}

pub fn _internal_list_iterate_(mut _self: LBObject, _arguments: LBObjArgs) -> LBObject {
    let mut borrowed = _self.clone();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_iterator(Box::new(LeblancVecIterator::new(list.internal_vec.clone())))
}
pub fn _internal_list_length_(_self: LBObject, _arguments: LBObjArgs) -> LBObject {
    let mut borrowed = _self;
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_int(list.internal_vec.len() as i32)
}

