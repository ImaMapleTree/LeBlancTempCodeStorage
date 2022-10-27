



use crate::leblanc::core::leblanc_object::{RustDataCast};
use crate::leblanc::core::native_types::promise_type::ArcLeblancPromise;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub fn _internal_promise_consume_(_self: LBObject, _arguments: LBObjArgs) -> LBObject {
    let mut borrowed = _self;
    let promise: &mut ArcLeblancPromise = borrowed.data.mut_data().unwrap();
    let x = promise.inner.write().consume().unwrap(); x
}