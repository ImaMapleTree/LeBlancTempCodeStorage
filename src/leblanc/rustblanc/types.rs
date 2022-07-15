use std::sync::Arc;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::strawberry::Strawberry;

pub type LBObject = Arc<Strawberry<LeBlancObject>>;
pub type BridgeModSetter = extern fn(CoreModule);
pub type BridgeObjSetter = extern fn(LeBlancObject);
pub type BridgeModGetter = extern fn() -> Option<&'static mut CoreModule>;
pub type BridgeObjGetter = extern fn() -> Option<&'static mut LeBlancObject>;
