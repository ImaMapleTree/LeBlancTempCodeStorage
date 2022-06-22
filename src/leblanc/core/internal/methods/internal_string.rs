use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;

pub fn _internal_add_string(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let string = _self.to_string();
    let string2 = arguments[0].to_string();
    return (string + &string2).create_mutex();
}