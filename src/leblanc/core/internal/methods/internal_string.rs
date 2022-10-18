

use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::LBObject;

pub fn _internal_add_string(_self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    let string = _self.to_string();
    let string2 = arguments[0].to_string();
    (string + &string2).create_mutex()
}