use std::alloc::{GlobalAlloc, Layout, System};
use std::ffi::c_void;
use std::mem::take;
use std::ptr::addr_of_mut;
use std::sync::Arc;
use typed_arena::Arena;
use libmimalloc_sys::mi_free;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::strawberry::Strawberry;

static mut CORE_MODULE: Arena<CoreModule> = Arena::new();
static mut SHARED_OBJECT: Arena<LeBlancObject> = Arena::new();

static mut OBJ_BRIDGE_SETTER: extern fn(LeBlancObject) = _unsafe_set_shared_object;
static mut MOD_BRIDGE_SETTER: extern fn(CoreModule) = _unsafe_set_module_export;

static mut OBJ_BRIDGE_GETTER: extern fn() -> Option<&'static mut LeBlancObject> = _unsafe_get_shared_object;
static mut MOD_BRIDGE_GETTER: extern fn() -> Option<&'static mut CoreModule> = _unsafe_get_module_export;

/*pub fn clear_mod() {
    unsafe {
        if CORE_MODULE.is_some() {
            println!("Boxing");
            Box::into_raw(Box::new(take(CORE_MODULE.as_mut().unwrap())));
            println!("Boxed");
            CORE_MODULE = None;
        }
    }
}

pub fn clear_obj() {
    unsafe {
        let mut obj = take(SHARED_OBJECT.as_mut().unwrap());
        let ptr = addr_of_mut!(obj);
        mi(ptr as *mut c_void);
        //System.dealloc(ptr as *mut u8, Layout::new::<LeBlancObject>());

        println!("Deconstructing type");
        obj.typing = LeBlancType::Null;
        println!("Deconstructing context");
        obj.context = VariableContext::empty();
        println!("Deconstructing data");
        obj.data = LeBlancObjectData::Null;
        println!("Deconstructing methods");
        obj.methods = LeBlancObject::unsafe_null().lock().methods.clone();
        println!("Deconstructing members");
        let members = take(obj.members.underlying_pointer());
        println!("Drop members");
        drop(members);
        println!("References: {}", Arc::strong_count(&obj.members));
        println!("Dropping arc");
        println!("Members: {:#?}", obj.members);
        let members = take(&mut obj.members);
        let p = Box::into_raw(Box::new(members));
        println!("Swapping members");
        obj.members = LeBlancObject::unsafe_null().lock().members.clone();
        drop(obj);

        println!("Taking obj");
        println!("Dereferencing");
        println!("Done");
        Box::from_raw(p);

    }
}
*/
pub fn set_obj_bridge(setter: extern fn(LeBlancObject), getter: extern fn() -> Option<&'static mut LeBlancObject>) {
    unsafe {
        OBJ_BRIDGE_SETTER = setter;
        OBJ_BRIDGE_GETTER = getter;
    }
}

pub fn set_mod_bridge(setter: extern fn(CoreModule), getter: extern fn() -> Option<&'static mut CoreModule>) {
    unsafe {
        MOD_BRIDGE_SETTER = setter;
        MOD_BRIDGE_GETTER = getter;
    }
}

pub fn obj_bridge_setter() -> extern fn(LeBlancObject) {
    unsafe { OBJ_BRIDGE_SETTER }
}

pub fn mod_bridge_setter() -> extern fn(CoreModule) {
    unsafe { MOD_BRIDGE_SETTER }
}

pub fn obj_bridge_getter() -> extern fn() -> Option<&'static mut LeBlancObject> {
    unsafe { OBJ_BRIDGE_GETTER }
}

pub fn mod_bridge_getter() -> extern fn() -> Option<&'static mut CoreModule> {
    unsafe { MOD_BRIDGE_GETTER }
}


#[macro_export]
macro_rules! bridge_setup {
    (Module: $func1:expr, $func2:expr) => {
        {
            unsafe { leblanc::leblanc::rustblanc::bridge::set_mod_bridge($func1, $func2) }
        }
    };
    (Object: $func1:expr, $func2:expr) => {
        {
            unsafe { leblanc::leblanc::rustblanc::bridge::set_obj_bridge($func1, $func2) }
        }
    }
}

#[macro_export]
macro_rules! bridge {
    (Module: $func:expr) => {
        {
            unsafe { leblanc::leblanc::rustblanc::bridge::mod_bridge_setter()($func) }
            leblanc::leblanc::rustblanc::bridge::mod_bridge_getter()()
        }
    };
    (Object: $func:expr) => {
        {
            unsafe { leblanc::leblanc::rustblanc::bridge::obj_bridge_setter()($func) }
            leblanc::leblanc::rustblanc::bridge::obj_bridge_getter()()
        }
    }
}

pub use bridge;
pub use bridge_setup;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::native_types::LeBlancType;

pub extern fn _unsafe_set_module_export(module: CoreModule) -> &'static mut CoreModule {
    unsafe { CORE_MODULE.alloc(module) }
}

pub extern fn _unsafe_set_shared_object(object: LeBlancObject) -> &'static mut LeBlancObject {
    unsafe { SHARED_OBJECT.alloc(object) }
}

/*pub extern fn _unsafe_get_module_export() -> Option<&'static mut CoreModule> {
    return unsafe {CORE_MODULE.as_mut()};
}

pub extern fn _unsafe_get_shared_object() -> Option<&'static mut LeBlancObject> {
    return unsafe {SHARED_OBJECT.as_mut()};
}*/