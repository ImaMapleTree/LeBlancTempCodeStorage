use std::collections::{HashMap, HashSet};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;

pub fn _internal_field_(_self: &LeBlancObject, arguments: &[LeBlancObject]) -> LeBlancObject {
    let string: String = unsafe {(arguments[0].reflect().downcast_ref_unchecked::<String>())}.clone();

    return _self.members.get(string.as_str()).unwrap_or(&LeBlancObject::null()).clone();
}

pub fn _internal_expose_(_self: &LeBlancObject, arguments: &[LeBlancObject]) -> LeBlancObject {
    let class_meta = ClassMeta::default("ExposedObject".to_string(), 0);
    let mut expose_object = LeBlancObject::new(
        LeBlancObjectData::Class(class_meta.clone()),
        LeBlancType::Class(class_meta.parse_id),
        base_methods(),
        HashMap::new(),
        VariableContext::empty(),
    );

    expose_object.members.insert("name".to_string(), leblanc_object_string(_self.name_of()));

    let variable_class_meta = ClassMeta::default("VariableContext".to_string(), 1);
    let mut variable_state = LeBlancObject::new(
        LeBlancObjectData::Class(variable_class_meta.clone()),
        LeBlancType::Class(variable_class_meta.parse_id),
        base_methods(),
        HashMap::new(),
        VariableContext::empty()
    );

    variable_state.members.insert("name".to_string(), leblanc_object_string(_self.context.name.to_string()));
    variable_state.members.insert("state".to_string(), leblanc_object_string(_self.context.state.to_string()));
    variable_state.members.insert("lineNumber".to_string(), leblanc_object_string(_self.context.line_number.to_string()));
    variable_state.members.insert("file".to_string(), leblanc_object_string(_self.context.file.to_string()));

    expose_object.members.insert("variableContext".to_string(), variable_state);


    return expose_object;
}