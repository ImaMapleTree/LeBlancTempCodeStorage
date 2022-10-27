





use crate::leblanc::core::leblanc_object::{Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub fn _internal_add_string(_self: LBObject, arguments: LBObjArgs) -> LBObject {
    let string = _self.to_string();
    let string2 = arguments[0].to_string();
    (string + &string2).create_mutex()
}