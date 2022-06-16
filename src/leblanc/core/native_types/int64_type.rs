use std::collections::{BTreeSet, HashMap};
use crate::leblanc::core::internal::methods::internal_math::_internal_inplace_add_;
use crate::leblanc::core::leblanc_argument::number_argset;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_int64(integer: i64) -> LeBlancObject {
    let mut base_methods = base_methods();
    base_methods.insert(inplace_addition());

    return LeBlancObject::new(
        LeBlancObjectData::Int64(integer),
        LeBlancType::Int64,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}


impl ToLeblanc for i64 {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_int64(*self);
    }
}

fn inplace_addition() -> Method {
    let method_store = MethodStore::new("inplace_addition".to_string(), number_argset());
    let mut method_tag = BTreeSet::new();
    method_tag.insert(MethodTag::InPlaceAddition);
    return Method::new(
        method_store,
        _internal_inplace_add_,
        method_tag
    )
}