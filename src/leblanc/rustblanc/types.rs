use std::sync::Arc;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::strawberry::Strawberry;

pub type LBObject = Arc<Strawberry<LeBlancObject>>;
pub type LBReturn = Option<&'static mut LeBlancObject>;
pub type ModReturn = Option<&'static mut CoreModule>;

pub type BIModFunc = extern fn(CoreModule);
pub type BIObjFunc = extern fn(LeBlancObject);

pub type BModGetter = extern fn() -> ModReturn;
pub type BObjGetter = extern fn() -> LBReturn;

pub type BModSwapper = extern fn(BIModFunc);
pub type BObjSwapper = extern fn(BIObjFunc);