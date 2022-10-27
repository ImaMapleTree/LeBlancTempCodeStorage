use crate::lbvm::{LBVM, LBVMResult};
use crate::lbvm::frame::Frame;
use crate::lbvm::ivalue::IVal;
use crate::lbvm::instructions::*;
use crate::lbvm::method::Method;
use crate::lbvm::vm_storage::Constant;
use crate::leblanc_type::LBValue;

pub(crate) fn execute_method(lbvm: &LBVM, method: &Method, args: &[IVal]) -> LBVMResult<Option<LBValue>> {
    Ok(None)
}


#[inline(always)]
pub(crate) fn execute_frame_until_end(lbvm: &LBVM, frame: &mut Frame) -> LBVMResult<Option<LBValue>> {
    loop {
        //println!("{:?}", frame.instructions);
        //println!("Frame pointer: {}", frame.fp);
        let instruct = frame.read_instr_u16();
        //println!("{} ({:?})", debug_instruct(instruct), vec![((instruct & 0xFF00) >> 8) as u8, (instruct & 0x00FF) as u8]);
        match instruct {
            NOP => {}
            NOTIMPL => {panic!("Not implemented")}
            ICONST0 => frame.push(IVal::from_int(0)),
            ICONST1 => frame.push(IVal::from_int(1)),
            ICONST2 => {
                //println!("Frame stack: {:?}", frame.stack);
                frame.push(IVal::from_int(2));
                //println!("Frame stack: {:?}", frame.stack);
            },
            ICONST3 => frame.push(IVal::from_int(3)),
            ICONST4 => frame.push(IVal::from_int(4)),
            ICONST5 => frame.push(IVal::from_int(5)),
            ICONST10 => frame.push(IVal::from_int(10)),
            LCONST0 => frame.push_long(0),
            LCONST1 => frame.push_long(1),
            LCONST2 => frame.push_long(2),
            FCONST0 => frame.push(IVal::from_float(0.0)),
            FCONST1 => frame.push(IVal::from_float(1.0)),
            FCONST2 => frame.push(IVal::from_float(2.0)),
            DCONST0 => frame.push_double(0.0),
            DCONST1 => frame.push_double(1.0),
            DCONST2 => frame.push_double(2.0),

            IINC => {
                let index = frame.read_instr_u16();
                let mut value =  unsafe {&mut *frame.locals.as_mut_ptr().add(index as usize)};
                value.0 += 1;
            }
            IADDSET => {
                let index = frame.read_instr_u16();
                let mut value =  unsafe {&mut *frame.locals.as_mut_ptr().add(index as usize)};
                value.0 += frame.pop().as_int() as usize;
                //println!("Value 0: {}", value.0);
            }
            INOT => {
                let v = frame.pop().as_int();
                let not = !v;
                //println!("!{} = {}", v, not);

                let val = IVal::from_int(not);
                frame.stack.push_quick(val);
            }
            IADD => {
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                frame.stack.push_quick(IVal::from_int(a + b));
            }
            ISUB => {
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                //println!("{} - {} = {}", a, b, a - b);
                frame.stack.push_quick(IVal::from_int(a - b));
            }
            IMUL => {
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                frame.stack.push_quick(IVal::from_int(a * b));
            }
            IDIV => {
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                frame.stack.push_quick(IVal::from_int(a / b));
            }
            IMOD =>  {
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                //println!("{} % {} = {}", a, b, a % b);
                frame.stack.push_quick(IVal::from_int(a % b));
            }
            LOADCONST => {
                let index = frame.read_instr_u16();
                match lbvm.storage.constants.get(index as usize) {
                    Some(Constant::Long(long)) => frame.push_long(*long),
                    Some(Constant::Double(double)) => frame.push_double(*double),
                    Some(Constant::Int(int)) => frame.push(IVal::from_int(*int)),
                    Some(Constant::Float(float)) => frame.push(IVal::from_float(*float)),
                    Some(Constant::Short(short)) => frame.push(IVal::from_short(*short)),
                    Some(Constant::String(string)) => todo!("Load strings from const pool"),
                    other => unreachable!()
                }
            }
            LOADCONST2 => {

            }
            LOADVAR => {
                let index = frame.read_instr_u16();
                let value =  unsafe {*frame.locals.as_ptr().add(index as usize)};
                //println!("Value: {:?}", value);
                frame.stack.push_quick(value);
                //println!("Load var stack: {:?}", frame.stack);
            }
            STOREVAR => {
                let index = frame.read_instr_u16();
                //println!("Frame stack: {:?}", frame.stack);
                let value = frame.stack.pop();
                //println!("Value: {:?}", value);
                //println!("Index: {:?}", index);
                frame.locals[index as usize] = value.unwrap();
            }
            CALLNORMAL => {
                let index = frame.read_instr_u16();
                let method = unsafe { &*lbvm.storage.methods.as_ptr().add(index as usize) };
                frame.execute_method(lbvm, method)?;
            }
            IFTRUE => {
                let jump = frame.read_instr_u16();
                let a = frame.pop().as_int();
                if !(a == 1 || a == -1) {
                    frame.fp += jump as usize;
                }
            }
            IFLT => {
                let jump = frame.read_instr_u16();
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                //println!("{} < {}", a, b);
                if !(a < b) {
                    frame.fp += jump as usize;
                }
            }
            IFLE => {
                let jump = frame.read_instr_u16();
                let b = frame.pop().as_int();
                let a = frame.pop().as_int();
                if !(a <= b) {
                    frame.fp += jump as usize;
                }
            }
            IRETURN => {
                return Ok(Some(LBValue::from(frame.pop().as_int())))
            }
            /// All returns are objects in this instruction
            RETURN => {
                let returns = frame.read_instr_u8();
            }
            NORETURN => {
                return Ok(None);
            }
            GOTO => {
                let fp = frame.read_instr_u16();
                frame.fp = fp as usize;
            }
            PRINT => {
                let args = frame.read_instr_u16();
                for _ in 0..args {
                    println!("{}", frame.pop().as_int())
                }
            }
            other => todo!("Bytecode: {}", other)
        }
    }
}
