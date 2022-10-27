use fxhash::{FxHashMap};








use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect, Stringify};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use crate::leblanc::rustblanc::memory::heap::HeapRef;

use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

/*pub fn _internal_field_(_self: LBObject, arguments: LBObjArgs) -> LBObject {
    let string: String = unsafe {arguments[0].reflect().downcast_ref_unchecked::<String>()}.clone();

    return _self.members.get(string.as_str()).unwrap_or(&LeBlancObject::null()).clone();
}

pub fn _internal_expose_(_self: LBObject, _arguments: LBObjArgs) -> LBObject {
    let class_meta = ClassMeta::default("ExposedObject".to_string(), 0);
    let mut expose_object = LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(class_meta.clone())),
        LeBlancType::Class(class_meta.name),
        base_methods(),
        HeapRef::default(),
        VariableContext::empty(),
    );

    expose_object.members.insert("name".to_string(), leblanc_object_string(_self.name_of()));

    let variable_class_meta = ClassMeta::default("VariableContext".to_string(), 1);
    let mut variable_state = LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(variable_class_meta.clone())),
        LeBlancType::Class(variable_class_meta.name),
        base_methods(),
        HeapRef::default(),
        VariableContext::empty()
    );

    variable_state.members.insert("name".to_string(), leblanc_object_string(_self.context.name.to_string()));
    variable_state.members.insert("state".to_string(), leblanc_object_string(_self.context.state.to_string()));
    variable_state.members.insert("lineNumber".to_string(), leblanc_object_string(_self.context.line_number.to_string()));
    variable_state.members.insert("file".to_string(), leblanc_object_string(_self.context.file.to_string()));

    expose_object.members.insert("variableContext".to_string(), variable_state);


    expose_object
}*/

pub fn _internal_to_string_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    _self.to_string().create_mutex()
}