use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::native_types::promise_type::ArcLeblancPromise;

pub fn _internal_promise_consume_(_self: Arc<Mutex<LeBlancObject>>, _arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let mut borrowed = _self.lock().unwrap();
    let promise: &mut ArcLeblancPromise = borrowed.data.mut_data().unwrap();
    let x = promise.inner.lock().unwrap().consume().unwrap(); x
}