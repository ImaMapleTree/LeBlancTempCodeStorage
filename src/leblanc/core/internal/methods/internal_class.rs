use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect, Stringify};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use crate::leblanc::rustblanc::blueberry::Quantum;
use crate::leblanc::rustblanc::types::LBObject;

pub fn _internal_field_(_self: LBObject, arguments: Vec<LBObject>) -> LBObject {
    let string: String = unsafe {arguments[0].reflect().downcast_ref_unchecked::<String>()}.clone();

    return _self.members.get(string.as_str()).unwrap_or(&LeBlancObject::null()).clone();
}

pub fn _internal_expose_(_self: LBObject, _arguments: Vec<LBObject>) -> LBObject {
    let class_meta = ClassMeta::default("ExposedObject".to_string(), 0);
    let mut expose_object = LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(class_meta.clone())),
        LeBlancType::Class(class_meta.name),
        base_methods(),
        FxHashMap::default(),
        VariableContext::empty(),
    );

    expose_object.members.insert("name".to_string(), leblanc_object_string(_self.name_of()));

    let variable_class_meta = ClassMeta::default("VariableContext".to_string(), 1);
    let mut variable_state = LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(variable_class_meta.clone())),
        LeBlancType::Class(variable_class_meta.name),
        base_methods(),
        FxHashMap::default(),
        VariableContext::empty()
    );

    variable_state.members.insert("name".to_string(), leblanc_object_string(_self.context.name.to_string()));
    variable_state.members.insert("state".to_string(), leblanc_object_string(_self.context.state.to_string()));
    variable_state.members.insert("lineNumber".to_string(), leblanc_object_string(_self.context.line_number.to_string()));
    variable_state.members.insert("file".to_string(), leblanc_object_string(_self.context.file.to_string()));

    expose_object.members.insert("variableContext".to_string(), variable_state);


    expose_object
}

pub fn _internal_to_string_(_self: LBObject, _args: Vec<LBObject>) -> LBObject {
    _self.to_string().create_mutex()
}