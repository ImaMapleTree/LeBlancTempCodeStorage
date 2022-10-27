use std::ptr;

pub mod copystring;
pub mod simplevec;
pub mod unsafe_vec;
pub mod unsafe_raw_vec;

pub trait UnsafePop<T> {
    unsafe fn pop_quick(&mut self) -> T;
}

impl<T> UnsafePop<T> for Vec<T> {
    unsafe fn pop_quick(&mut self) -> T {
        let len = self.len() - 1;
        self.set_len(len);
        ptr::read(self.as_ptr().add(self.len()))
    }
}
