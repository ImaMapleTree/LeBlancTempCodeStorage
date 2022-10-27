use fxhash::{FxHashMap, FxHashSet};








use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub fn leblanc_object_float(float: f32) -> LBObject {
    let base_methods = base_methods();

    LeBlancObject::new(
        LeBlancObjectData::Float(float),
        7,
        UnsafeVec::default()
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
        leblanc_object_float(*self)._clone()
    }
    fn create_mutex(&self) -> LBObject { leblanc_object_float(*self) }
}