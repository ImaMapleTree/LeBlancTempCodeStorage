use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use fxhash::FxHashMap;

static mut EMPTY_MEMBERS: Option<Arc<Mutex<FxHashMap<String, LeBlancObject>>>> = None;

pub fn unsafe_empty_members() -> Arc<Mutex<FxHashMap<String, LeBlancObject>>>{
    unsafe {
        match &EMPTY_MEMBERS {
            Some(members) => members.clone(),
            None => {
                EMPTY_MEMBERS = Some(Arc::new(Mutex::new(FxHashMap::default())));
                EMPTY_MEMBERS.as_ref().unwrap().clone()
            }
        }
    }
}