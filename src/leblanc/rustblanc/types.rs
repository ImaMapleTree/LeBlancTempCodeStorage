
use crate::leblanc::core::interpreter::execution_context::ExecutionContext;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::rustblanc::blueberry::{Blueberry};
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

pub type IExec = fn(&mut ExecutionContext, Instruction2) -> IExecResult;
pub type IExecResult = Result<(), LBObject>;

pub type LBFunctionHandle = fn(LBObject, LBObjArgs) -> LBObject;

pub type LBObject = HeapRef<'static, LeBlancObject>;
pub type LBObjArgs = UnsafeVec<LBObject>;

pub type ObjMembers = HeapRef<'static, Vec<LBObject>>;

pub type LBObject2 = Blueberry<'static, LeBlancObject>;
pub type LBReturn = Option<&'static mut LeBlancObject>;






/*pub type ModReturn = Option<&'static mut CoreModule>;
pub type BIModFunc = extern fn(CoreModule);
pub type BIObjFunc = extern fn(LeBlancObject);

pub type BModGetter = extern fn() -> ModReturn;
pub type BObjGetter = extern fn() -> LBReturn;

pub type BModSwapper = extern fn(BIModFunc);
pub type BObjSwapper = extern fn(BIObjFunc);*/

