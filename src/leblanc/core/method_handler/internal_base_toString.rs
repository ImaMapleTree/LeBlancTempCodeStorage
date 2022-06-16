use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use crate::LeBlancType;

pub fn INTERNAL_TO_STRING(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    return Arc::new(Mutex::new(leblanc_object_string(_self.lock().unwrap().data.to_string())));
}