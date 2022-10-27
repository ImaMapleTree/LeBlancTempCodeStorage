use crate::lbvm::frame::Frame;
use crate::lbvm::ivalue::IVal;
use crate::lbvm::LBVM;

pub(crate) fn print_handle(lbvm: &LBVM, args: &[IVal]) {
    for arg in args {
        println!("{}", arg.as_int())
    }
}