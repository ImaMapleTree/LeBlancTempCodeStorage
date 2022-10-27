use std::ptr;
use crate::lbvm::ivalue::IVal;
use crate::lbvm::{LBVM, LBVMResult};

use crate::lbvm::method::Method;
use crate::leblanc::simplevec::SimpleVec;
use crate::leblanc::unsafe_vec::UnsafeVec;
use crate::leblanc::UnsafePop;
use crate::leblanc_type::LBValue;

pub(crate) struct Frame<'a> {
    pub(crate) method: &'a Method,
    // LB instructions are u16 with varied u16 / u8 arguments
    pub(crate) instructions: *const u8,
    pub(crate) locals: Vec<IVal>,
    pub(crate) stack: &'a mut UnsafeVec<IVal>,
    pub(crate) fp: usize
}

impl<'a> Frame<'_> {
    pub fn read_instr_u8(&mut self) -> u8 {
        let byte = unsafe { *(self.instructions.add(self.fp)) };
        self.fp += 1;
        byte
    }

    pub fn read_instr_u16(&mut self) -> u16 {
        let byte = unsafe {*(self.instructions.add(self.fp) as *mut u16)};
        self.fp += 2;
        byte
    }

    pub fn read_instr_u32(&mut self) -> u32 {
        let byte = unsafe {*(self.instructions.add(self.fp) as *mut u32)};
        self.fp += 4;
        byte
    }

    // TODO: possible switch with UnsafeVec
    pub fn pop(&mut self) -> IVal {
        unsafe { self.stack.pop_unsafe() }
    }

    pub fn pop_long(&mut self) -> i64 {
        let (high, low) = (self.pop(), self.pop());
        IVal::as_long(low, high)
    }

    pub fn pop_double(&mut self) -> f64 {
        let (high, low) = (self.pop(), self.pop());
        IVal::as_double(low, high)
    }

    pub fn push(&mut self, value: IVal) {
        self.stack.push_quick(value);
    }

    pub fn push_long(&mut self, value: i64) {
        let (low, high) = IVal::from_long(value);
        self.stack.push_quick(low);
        self.stack.push_quick(high);
    }

    pub fn push_double(&mut self, value: f64) {
        let (low, high) = IVal::from_double(value);
        self.stack.push_quick(low);
        self.stack.push_quick(high);
    }

    pub fn execute_method(&mut self, lbvm: &LBVM, method: &Method) -> LBVMResult<()> {
        let result = method.execute_new_frame(lbvm, self)?;
        match result {
            None => (),
            Some(LBValue::Long(long)) => self.push_long(long),
            Some(LBValue::Double(double)) => self.push_double(double),
            Some(LBValue::Int(int)) => self.push(IVal::from_int(int)),
            Some(LBValue::Float(float)) => self.push(IVal::from_float(float)),
            Some(LBValue::Short(short)) => self.push(IVal::from_short(short)),
            Some(LBValue::Ref(string)) => todo!("more object stuff smile"),
        };
        Ok(())
    }

}

