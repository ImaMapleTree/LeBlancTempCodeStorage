use std::alloc::Layout;
use std::ptr;
use crate::Heap;
use crate::lbvm::heap::LBptr;
use crate::lbvm::ivalue::IVal;
use crate::lbvm::object::Object;
use crate::lbvm::vm_storage::VmStorage;
use crate::leblanc::simplevec::SimpleVec;

pub(crate) mod instructions;
pub(crate) mod heap;
pub(crate) mod class;
pub(crate) mod method;
pub(crate) mod vm_storage;
pub(crate) mod interpreter;
pub(crate) mod frame;
pub(crate) mod exception_handler;
pub mod object;
pub mod ivalue;

pub type LBVMResult<T> = Result<T, Object>;


pub struct LBVM {
    // Randomized uuid specific to this lbvm
    pub(crate) id: usize,

    pub(crate) heap: &'static mut Heap,

    pub(crate) storage: VmStorage,

    pub(crate) stack: SimpleVec<IVal>
}
impl LBVM {
    fn heap(&self) -> &mut Heap {
        unsafe {&mut *(self.heap as *const Heap as *mut Heap)}
    }
}

impl Default for LBVM {


    fn default() -> Self {
        LBVM {
            id: 0,
            heap: Box::leak(Box::new(Heap::new(1024 * 4096, 4096))),
            storage: VmStorage::default(),
            stack: SimpleVec::new()
        }
    }
}