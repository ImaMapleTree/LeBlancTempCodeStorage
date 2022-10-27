use std::env::args;
use crate::lbvm::exception_handler::ExceptionHandler;
use crate::lbvm::frame::Frame;
use crate::lbvm::interpreter::execute_frame_until_end;
use crate::lbvm::{LBVM, LBVMResult};
use crate::lbvm::ivalue::IVal;
use crate::leblanc::copystring::CopyString;
use crate::leblanc_type::{LBType, LBValue};

pub(crate) fn null_handle(_: &LBVM, _: &[IVal]) -> LBVMResult<Option<LBValue>> {Ok(None)}

pub(crate) struct Builtin(fn(&LBVM, &[IVal]) -> LBVMResult<Option<LBValue>>);

impl Default for Builtin {
    fn default() -> Self {
        Self(null_handle)
    }
}

pub(crate) struct Method {
    pub(crate) name: CopyString,
    pub(crate) args: Vec<LBType>,
    pub(crate) slots: usize,
    pub(crate) instructs: &'static [u8],
    pub(crate) handle: Builtin,
    pub(crate) handler: ExceptionHandler,
    pub(crate) locals: usize
}

impl Method {
    pub fn arg_slots(&self) -> usize {
        self.args.iter().map(|typ| match typ {
            LBType::Double | LBType::Long => 2,
            _ => 1
        }).sum()
    }

    pub fn execute_new_frame(&self, lbvm: &LBVM, frame: &mut Frame) -> LBVMResult<Option<LBValue>> {
        let slice = frame.stack.len() - self.slots;
        //println!("Frame: {:?}", frame.stack);
        let mut args = frame.stack.split_off_as_vec(slice, self.locals);
        //println!("Frame SLOCS IJE: {:?}", frame.stack);
        if slice == 0 {

        }
        //println!("Args: {:?}", args);
        unsafe { args.set_len(self.locals); }
        let mut new_frame = Frame { method: self, instructions: self.instructs.as_ptr(), locals: args, stack: frame.stack, fp: 0 };
        let result = execute_frame_until_end(lbvm, &mut new_frame)?;
        drop(new_frame);
        Ok(result)
    }
}