use std::collections::HashMap;

use crate::leblanc::rustblanc::strawberry::Strawberry;
use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_int(integer: i32) -> LeBlancObject {
    let base_methods = base_methods();


    return LeBlancObject::new(
        LeBlancObjectData::Int(integer),
        LeBlancType::Int,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}

/*fn int_addition_method() -> Method {
    let method_store = MethodStore {
        name: "addition".to_string(),
        arguments: number_argset(),
    };
    let mut method_tag = BTreeSet::new();
    method_tag.insert(MethodTag::Addition);

    return Method::new(
        method_store,
        _internal_add_number_,
        method_tag
    )
}*/

impl ToLeblanc for i32 {
    fn create(&self) -> LeBlancObject { return leblanc_object_int(*self); }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { return Rc::new(RefCell::new(self.create())) }
}