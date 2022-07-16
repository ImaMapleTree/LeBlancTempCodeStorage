use std::ptr::replace;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::module::CoreModule;


static mut CORE_MODULE: Option<CoreModule> = None;
static mut SHARED_OBJECT: Option<LeBlancObject> = None;

static mut OBJ_BRIDGE_SETTER: BIObjFunc = _unsafe_set_shared_object;
static mut MOD_BRIDGE_SETTER: BIModFunc = _unsafe_set_module_export;

static mut OBJ_BRIDGE_GETTER: BObjGetter = _unsafe_get_shared_object;
static mut MOD_BRIDGE_GETTER: BModGetter = _unsafe_get_module_export;

static mut MOD_SWAPPER: BIModFunc = _swap_current_module_export;
static mut OBJ_SWAPPER: BIObjFunc = _swap_current_shared_object;


pub fn set_obj_bridge(setter: BIObjFunc, getter: BObjGetter) {
    unsafe {
        OBJ_BRIDGE_SETTER = setter;
        OBJ_BRIDGE_GETTER = getter;
    }
}

pub fn set_mod_bridge(setter: BIModFunc, getter: BModGetter) {
    unsafe {
        MOD_BRIDGE_SETTER = setter;
        MOD_BRIDGE_GETTER = getter;
    }
}

pub extern fn set_obj_swapper(swapper: BIObjFunc) {
    unsafe {
        OBJ_SWAPPER = swapper;
    }
}

pub extern fn set_mod_swapper(swapper: BIModFunc) {
    unsafe {
        MOD_SWAPPER = swapper;
    }
}

pub fn mod_swapper() -> BIModFunc {
    unsafe { MOD_SWAPPER }
}


pub fn obj_bridge_setter() -> BIObjFunc {
    unsafe { OBJ_BRIDGE_SETTER }
}

pub fn mod_bridge_setter() -> BIModFunc {
    unsafe { MOD_BRIDGE_SETTER }
}

pub fn obj_bridge_getter() -> BObjGetter {
    unsafe { OBJ_BRIDGE_GETTER }
}

pub fn mod_bridge_getter() -> BModGetter {
    unsafe { MOD_BRIDGE_GETTER }
}


#[macro_export]
macro_rules! bridge_setup {
    (Module: $func1:expr, $func2:expr, $func3:expr) => {
        {
            unsafe { leblanc::leblanc::rustblanc::bridge::set_mod_bridge($func1, $func2) }
            $func3(leblanc::leblanc::rustblanc::bridge::_swap_current_module_export);
        }
    };
    (Object: $func1:expr, $func2:expr, $func3:expr) => {
        {
            unsafe { leblanc::leblanc::rustblanc::bridge::set_obj_bridge($func1, $func2) }
            $func3(leblanc::leblanc::rustblanc::bridge::_swap_current_shared_object);
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
use crate::leblanc::rustblanc::types::{BIModFunc, BIObjFunc, BModGetter, BObjGetter};


pub extern fn _unsafe_set_module_export(module: CoreModule) {
    unsafe {
        if CORE_MODULE.is_some() {
            MOD_SWAPPER(module);
        } else {
            CORE_MODULE = Some(module)
        }
    }
}

pub extern fn _unsafe_set_shared_object(object: LeBlancObject) {
    unsafe {
        if SHARED_OBJECT.is_some() {
            OBJ_SWAPPER(object);
        } else {
            SHARED_OBJECT = Some(object)
        }
    }
}

pub extern fn _unsafe_get_module_export() -> Option<&'static mut CoreModule> {
    return unsafe {CORE_MODULE.as_mut()};
}

pub extern fn _unsafe_get_shared_object() -> Option<&'static mut LeBlancObject> {
    return unsafe {SHARED_OBJECT.as_mut()};
}

pub extern fn _swap_current_module_export(new_mod: CoreModule) {
    unsafe {

        // What's going on here?
        // We currently have memory stored in Rust's heap that's been allocated by a linked library.
        // This by-itself should not be an issue but when that memory goes to be dropped, LeBlanc's global allocator (mimalloc)
        // Is going to have a major issue since linked libraries allocate with system allocator
        // So we swap the value in memory with our new value then drop it using the correct allocator
        //
        // This function is stored (and pointed to) within Rust's heap

        drop(replace(mod_bridge_getter()().unwrap(), new_mod));
    }
}

pub extern fn _swap_current_shared_object(new_object: LeBlancObject) {
    unsafe {
        drop(replace(obj_bridge_getter()().unwrap(), new_object));
    }
}