use std::ptr::null_mut;
use std::time::Instant;
use crate::lbvm::exception_handler::ExceptionHandler;
use crate::lbvm::frame::Frame;
use crate::lbvm::heap::LBptr;
use crate::lbvm::instructions::{CALLNORMAL, GOTO, IADDSET, IFLT, IINC, IRETURN, LOADCONST, LOADVAR, NORETURN, PRINT, STOREVAR};
use crate::lbvm::interpreter::execute_frame_until_end;
use crate::lbvm::ivalue::IVal;
use crate::lbvm::LBVM;
use crate::lbvm::method::{Builtin, Method, null_handle};
use crate::lbvm::vm_storage::Constant;
use crate::leblanc::copystring::CopyString;
use crate::leblanc::simplevec::SimpleVec;
use crate::leblanc::unsafe_vec::UnsafeVec;
use crate::leblanc_type::LBType;
use crate::test::fib::fib_bytecode;
use crate::test::prime::prime_bytecode;


pub(crate) fn setup() {
    let mut lbvm = LBVM::default();

    let bytecode = prime_bytecode();


    let main = Method {
        name: CopyString::new("main"),
        args: vec![],
        slots: 0,
        handle: Builtin::default(),
        instructs: main_prime().leak(),
        handler: ExceptionHandler {},
        locals: 2
    };


    let method = Method {
        name: CopyString::new("fib"),
        args: vec![],
        slots: 1,
        handle: Builtin::default(),
        instructs: bytecode.leak(),
        handler: ExceptionHandler {},
        locals: 2
    };

    lbvm.storage.methods.push(method);

    lbvm.storage.methods.push(main);
    //fib_constants(&mut lbvm);
    prime_constants(&mut lbvm);

    let main = unsafe { &*lbvm.storage.methods.as_ptr().add(1) };

    let mut stack = UnsafeVec::new();

    let mut frame = Frame { method: main, instructions: main.instructs.as_ptr(), locals: vec![IVal::from_int(0); main.locals], stack: &mut stack, fp: 0 };

    let now = Instant::now();
    let result = execute_frame_until_end(&lbvm, &mut frame).unwrap();
    println!("{:?}", result);
    let elapsed = now.elapsed();
    println!("Total Elapsed: {}", elapsed.as_secs_f64());
}

pub fn real_fib(a: i32) -> i32 {
    if a <= 1 { return a; }
    real_fib(a - 2) + real_fib(a - 1)
}

fn fib_constants(lbvm: &mut LBVM) {
    lbvm.storage.constants.push(Constant::Int(1));
    lbvm.storage.constants.push(Constant::Int(2));
    lbvm.storage.constants.push(Constant::Int(30))
}

fn prime_constants(lbvm: &mut LBVM) {
    lbvm.storage.constants.push(Constant::Int(2));
    lbvm.storage.constants.push(Constant::Int(0));
    lbvm.storage.constants.push(Constant::Int(1));
    lbvm.storage.constants.push(Constant::Int(17));
    lbvm.storage.constants.push(Constant::Int(25001));
}



fn main_fib() -> Vec<u8> {
    [
        LOADCONST, 2,
        CALLNORMAL, 0,
        PRINT, 1,
        NORETURN
    ].iter().flat_map(|v| {
        let v = *v;
        vec![(v & 0x00FF) as u8, ((v & 0xFF00) >> 8) as u8]
    }).collect()
}

fn main_prime() -> Vec<u8> {
    [
        LOADCONST, 1,
        STOREVAR, 0,
        LOADCONST, 0,
        STOREVAR, 1,
        LOADVAR, 1,
        LOADCONST, 4,
        IFLT, 20,
        LOADVAR, 1,
        CALLNORMAL, 0,
        IADDSET, 0,
        IINC, 1,
        GOTO, 16,
        LOADVAR, 0,
        PRINT, 1,
        NORETURN,
    ].iter().flat_map(|v| {
        let v = *v;
        vec![(v & 0x00FF) as u8, ((v & 0xFF00) >> 8) as u8]
    }).collect()
}
