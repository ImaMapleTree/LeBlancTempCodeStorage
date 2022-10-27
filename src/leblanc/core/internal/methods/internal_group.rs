use std::ops::Deref;
use crate::leblanc::core::leblanc_object::{RustDataCast};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::group_type::LeblancGroup;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub fn _internal_group_apply_(_self: LBObject, mut _arguments: LBObjArgs) -> LBObject {
    let mut borrowed = _self;
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.apply(_arguments[0].clone(), _arguments.split_off(1));
    true.create_mutex()
}

pub fn _internal_group_pipe_(_self: LBObject, _arguments: LBObjArgs) -> LBObject {
    let mut borrowed = _self;
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.pipe(_arguments);
    true.create_mutex()
}

pub fn _internal_group_pipe_async_(_self: LBObject, _arguments: LBObjArgs) -> LBObject {
    let mut borrowed = _self;
    let group: &mut LeblancGroup = borrowed.data.mut_data().unwrap();
    group.pipe_async(_arguments);
    true.create_mutex()
}