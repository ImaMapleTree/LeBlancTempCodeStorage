use fxhash::{FxHashMap};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, Reflect, Stringify};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;

pub fn _internal_field_(_self: Rc<RefCell<LeBlancObject>>, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let string: String = unsafe {arguments[0].reflect().downcast_ref_unchecked::<String>()}.clone();

    return _self.borrow().members.lock().unwrap().get(string.as_str()).unwrap_or(&LeBlancObject::null()).clone().to_mutex();
}

pub fn _internal_expose_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let class_meta = ClassMeta::default("ExposedObject".to_string(), 0);
    let expose_object = LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(class_meta.clone())),
        LeBlancType::Class(class_meta.parse_id),
        base_methods(),
        Arc::new(Mutex::new(FxHashMap::default())),
        VariableContext::empty(),
    );

    expose_object.members.lock().unwrap().insert("name".to_string(), leblanc_object_string(_self.borrow().name_of()));

    let variable_class_meta = ClassMeta::default("VariableContext".to_string(), 1);
    let variable_state = LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(variable_class_meta.clone())),
        LeBlancType::Class(variable_class_meta.parse_id),
        base_methods(),
        Arc::new(Mutex::new(FxHashMap::default())),
        VariableContext::empty()
    );

    variable_state.members.lock().unwrap().insert("name".to_string(), leblanc_object_string(_self.borrow().context.name.to_string()));
    variable_state.members.lock().unwrap().insert("state".to_string(), leblanc_object_string(_self.borrow().context.state.to_string()));
    variable_state.members.lock().unwrap().insert("lineNumber".to_string(), leblanc_object_string(_self.borrow().context.line_number.to_string()));
    variable_state.members.lock().unwrap().insert("file".to_string(), leblanc_object_string(_self.borrow().context.file.to_string()));

    expose_object.members.lock().unwrap().insert("variableContext".to_string(), variable_state);


    expose_object.to_mutex()
}

pub fn _internal_to_string_(_self: Rc<RefCell<LeBlancObject>>, _args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    _self.to_string().create_mutex()
}