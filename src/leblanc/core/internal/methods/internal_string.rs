
use crate::leblanc::rustblanc::strawberry::Strawberry;
use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _internal_add_string(_self: Rc<RefCell<LeBlancObject>>, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let string = _self.to_string();
    let string2 = arguments[0].to_string();
    return (string + &string2).create_mutex();
}