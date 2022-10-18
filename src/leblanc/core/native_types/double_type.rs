use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

pub fn leblanc_object_double(double: f64) -> LBObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Double(double),
        LeBlancType::Double,
        base_methods,
        FxHashMap::default(),
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
        _internal_add_double_,
        method_tag
    )
}*/


impl ToLeblanc for f64 {
    fn create(&self) -> LeBlancObject {
        leblanc_object_double(*self)._clone()
    }
    fn create_mutex(&self) -> LBObject { leblanc_object_double(*self) }
}

impl RustDataCast<f64> for LeBlancObjectData {
    fn clone_data(&self) -> Option<f64> {
        match self {
            LeBlancObjectData::Double(fp) => Some(*fp),
            _ => None,
        }
    }

    fn ref_data(&self) -> Option<&f64> {
        match self {
            LeBlancObjectData::Double(fp) => Some(fp),
            _ => None,
        }
    }

    fn mut_data(&mut self) -> Option<&mut f64> {
        match self {
            LeBlancObjectData::Double(fp) => Some(fp),
            _ => None,
        }
    }
}