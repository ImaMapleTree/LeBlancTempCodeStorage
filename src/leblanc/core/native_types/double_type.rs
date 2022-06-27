use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_double(double: f64) -> LeBlancObject {
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
        leblanc_object_double(*self)
    }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { Rc::new(RefCell::new(self.create())) }
}