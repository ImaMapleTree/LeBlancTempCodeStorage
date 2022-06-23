use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect, Stringify};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;

pub fn _internal_field_(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let string: String = unsafe {arguments[0].lock().unwrap().reflect().downcast_ref_unchecked::<String>()}.clone();

    return Arc::new(Mutex::new(_self.lock().unwrap().members.get(string.as_str()).unwrap_or(&LeBlancObject::null()).clone()));
}

pub fn _internal_expose_(_self: Arc<Mutex<LeBlancObject>>, arguments: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    let class_meta = ClassMeta::default("ExposedObject".to_string(), 0);
    let mut expose_object = LeBlancObject::new(
        LeBlancObjectData::Class(class_meta.clone()),
        LeBlancType::Class(class_meta.parse_id),
        base_methods(),
        HashMap::new(),
        VariableContext::empty(),
    );

    expose_object.members.insert("name".to_string(), leblanc_object_string(_self.lock().unwrap().name_of()));

    let variable_class_meta = ClassMeta::default("VariableContext".to_string(), 1);
    let mut variable_state = LeBlancObject::new(
        LeBlancObjectData::Class(variable_class_meta.clone()),
        LeBlancType::Class(variable_class_meta.parse_id),
        base_methods(),
        HashMap::new(),
        VariableContext::empty()
    );

    variable_state.members.insert("name".to_string(), leblanc_object_string(_self.lock().unwrap().context.name.to_string()));
    variable_state.members.insert("state".to_string(), leblanc_object_string(_self.lock().unwrap().context.state.to_string()));
    variable_state.members.insert("lineNumber".to_string(), leblanc_object_string(_self.lock().unwrap().context.line_number.to_string()));
    variable_state.members.insert("file".to_string(), leblanc_object_string(_self.lock().unwrap().context.file.to_string()));

    expose_object.members.insert("variableContext".to_string(), variable_state);


    return Arc::new(Mutex::new(expose_object));
}

pub fn _internal_to_string_(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    _self.to_string().create_mutex()
}