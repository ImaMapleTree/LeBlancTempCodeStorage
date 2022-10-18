use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use fxhash::FxHashMap;
use lazy_static::lazy_static;
use crate::leblanc::core::native_types::rust_type::RustType;

lazy_static! {
    pub static ref EMPTY_MEMBERS: FxHashMap<String, LeBlancObject> = FxHashMap::default();
}