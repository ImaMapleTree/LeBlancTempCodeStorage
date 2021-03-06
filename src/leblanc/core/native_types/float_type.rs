use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_float(float: f32) -> LeBlancObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Float(float),
        LeBlancType::Float,
        base_methods,
        Arc::new(Strawberry::new(FxHashMap::default())),
        VariableContext::empty(),
    )
}

/*fn int_addition_method() -> Method {
    let method_store = MethodStore {
        name: "addition".to_string(),
        arguments: number_argset()
    };
    let mut method_tag = BTreeSet::new();
    method_tag.insert(MethodTag::Addition);

    return Method::new(
        method_store,
        _internal_add_float_,
        method_tag
    )
}*/


impl ToLeblanc for f32 {
    fn create(&self) -> LeBlancObject {
        leblanc_object_float(*self)
    }
    fn create_mutex(&self) -> Arc<Strawberry<LeBlancObject>> { Arc::new(Strawberry::new(self.create())) }
}