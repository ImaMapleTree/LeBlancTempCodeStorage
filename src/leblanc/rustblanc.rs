
use crate::leblanc::rustblanc::hex::Hexadecimal;
pub(crate) mod lib;
pub mod rust_override;
pub mod relationship;
pub mod utils;
pub mod hex;
pub mod copystring;
pub mod exception;
pub mod strawberry;

pub trait Appendable<T> {
    fn append_item(&mut self, item: T);
}

pub trait AppendCloneable<T: Clone> {
    fn append_clone(&mut self, item: &T);
}

pub trait Hexable {
    fn to_hex(&self, bytes: usize) -> Hexadecimal;

    fn from_hex(hex: &Hexadecimal) -> Self;
}

impl<T> Appendable<T> for Vec<T> {
    fn append_item(&mut self, item: T) {
        self.insert(self.len(), item);
    }
}

impl<T: Clone> AppendCloneable<T> for Vec<T> {
    fn append_clone(&mut self, item: &T) {
        self.insert(self.len(), item.clone());
    }
}