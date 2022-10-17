use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::native_types::promise_type::ArcLeblancPromise;
use crate::leblanc::rustblanc::types::LBObject;

pub fn _internal_promise_consume_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let borrowed = _self.underlying_pointer();
    let promise: &mut ArcLeblancPromise = borrowed.data.mut_data().unwrap();
    let x = promise.inner.write().consume().unwrap(); x
}