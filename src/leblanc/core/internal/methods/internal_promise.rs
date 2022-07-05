use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::native_types::promise_type::ArcLeblancPromise;

pub fn _internal_promise_consume_(_self: Arc<Strawberry<LeBlancObject>>, _arguments: &mut [Arc<Strawberry<LeBlancObject>>]) -> Arc<Strawberry<LeBlancObject>> {
    let mut borrowed = _self.lock();
    let promise: &mut ArcLeblancPromise = borrowed.data.mut_data().unwrap();
    let x = promise.inner.lock().consume().unwrap(); x
}