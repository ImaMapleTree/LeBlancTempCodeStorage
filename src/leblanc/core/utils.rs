use crate::leblanc::core::interpreter::leblanc_runner::HANDLES;
use crate::leblanc::rustblanc::types::{LBObjArgs, LBObject};

pub fn get_associated_function(name: String) {}

pub fn call_global_by_name(name: &str, args: LBObjArgs) -> Option<LBObject> {
    unsafe { &mut HANDLES }.iter_mut().find(|h| h.name == name).map(|a| a.execute(args))
}