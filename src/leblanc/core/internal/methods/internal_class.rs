use std::collections::HashMap;

use crate::leblanc::rustblanc::strawberry::{Either, Strawberry};

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect, Stringify};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;

pub fn _internal_field_(_self: Strawberry<LeBlancObject>, arguments: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    let string: String = unsafe {arguments[0].loan().inquire().either().reflect().downcast_ref_unchecked::<String>()}.clone();

    return _self.loan().inquire().either().members.get(string.as_str()).unwrap_or(&LeBlancObject::null()).clone().to_mutex();
}

pub fn _internal_expose_(_self: Strawberry<LeBlancObject>, _arguments: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    let class_meta = ClassMeta::default("ExposedObject".to_string(), 0);
    let mut expose_object = LeBlancObject::new(
        LeBlancObjectData::Class(class_meta.clone()),
        LeBlancType::Class(class_meta.parse_id),
        base_methods(),
        HashMap::new(),
        VariableContext::empty(),
    );

    expose_object.members.insert("name".to_string(), leblanc_object_string(_self.loan().inquire().either().name_of()));

    let variable_class_meta = ClassMeta::default("VariableContext".to_string(), 1);
    let mut variable_state = LeBlancObject::new(
        LeBlancObjectData::Class(variable_class_meta.clone()),
        LeBlancType::Class(variable_class_meta.parse_id),
        base_methods(),
        HashMap::new(),
        VariableContext::empty()
    );

    variable_state.members.insert("name".to_string(), leblanc_object_string(_self.loan().inquire().either().context.name.to_string()));
    variable_state.members.insert("state".to_string(), leblanc_object_string(_self.loan().inquire().either().context.state.to_string()));
    variable_state.members.insert("lineNumber".to_string(), leblanc_object_string(_self.loan().inquire().either().context.line_number.to_string()));
    variable_state.members.insert("file".to_string(), leblanc_object_string(_self.loan().inquire().either().context.file.to_string()));

    expose_object.members.insert("variableContext".to_string(), variable_state);


    return expose_object.to_mutex()
}

pub fn _internal_to_string_(_self: Strawberry<LeBlancObject>, _args: &mut [Strawberry<LeBlancObject>]) -> Strawberry<LeBlancObject> {
    _self.to_string().create_mutex()
}