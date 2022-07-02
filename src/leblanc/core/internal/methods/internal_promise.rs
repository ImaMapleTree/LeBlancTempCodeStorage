use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::native_types::promise_type::ArcLeblancPromise;

pub fn _internal_promise_consume_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let promise: &mut ArcLeblancPromise = borrowed.data.mut_data().unwrap();
    let x = promise.inner.lock().unwrap().consume().unwrap(); x
}