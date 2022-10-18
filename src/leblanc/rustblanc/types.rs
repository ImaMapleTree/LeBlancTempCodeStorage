use std::sync::Arc;
use arrayvec::ArrayVec;
use crate::leblanc::configuration::SSM_KB;
use crate::leblanc::core::interpreter::execution_context::ExecutionContext;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::module::CoreModule;
use crate::leblanc::rustblanc::blueberry::{Blueberry, Quantum};
use crate::leblanc::rustblanc::strawberry::Strawberry;

pub type LeBlancStack = ArrayVec<LBObject, { (SSM_KB * 1000) / 200 }>;
pub type IExec = fn(&mut ExecutionContext, Instruction2) -> IExecResult;
pub type IExecResult = Result<(), LBObject>;

pub type LBFunctionHandle = fn(LBObject, Vec<LBObject>) -> LBObject;

pub type LBObject = Quantum<LeBlancObject>;
pub type LBObject2 = Blueberry<'static, LeBlancObject>;
pub type LBReturn = Option<&'static mut LeBlancObject>;
pub type ModReturn = Option<&'static mut CoreModule>;

pub type BIModFunc = extern fn(CoreModule);
pub type BIObjFunc = extern fn(LeBlancObject);

pub type BModGetter = extern fn() -> ModReturn;
pub type BObjGetter = extern fn() -> LBReturn;

pub type BModSwapper = extern fn(BIModFunc);
pub type BObjSwapper = extern fn(BIObjFunc);