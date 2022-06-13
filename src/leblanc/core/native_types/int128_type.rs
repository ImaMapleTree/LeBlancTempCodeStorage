use std::collections::{BTreeSet, HashMap};
use crate::leblanc::core::internal::methods::internal_math::_internal_add_number_;
use crate::leblanc::core::leblanc_argument::{LeBlancArgument, number_argset};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;

pub fn leblanc_object_int128(integer: i128) -> LeBlancObject {
    let mut base_methods = base_methods();
    base_methods.insert(int_addition_method());

    return LeBlancObject::new(
        LeBlancObjectData::Int128(integer),
        LeBlancType::Int128,
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}

fn int_addition_method() -> Method {
    let method_store = MethodStore {
        name: "addition".to_string(),
        arguments: number_argset()
    };
    let mut method_tag = BTreeSet::new();
    method_tag.insert(MethodTag::Addition);

    return Method::new(
        method_store,
        _internal_add_number_,
        method_tag
    )
}

impl ToLeblanc for i128 {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_int128(*self);
    }
}