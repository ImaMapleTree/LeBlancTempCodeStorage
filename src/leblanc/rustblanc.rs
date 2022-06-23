use std::u16;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::utils::{decode_hex, encode_hex};

pub mod exception;
pub(crate) mod lib;
pub mod basic;
pub mod rust_override;
pub mod generic_data;
pub mod relationship;
pub mod utils;
pub mod hex;
pub mod packed_number;
pub mod berry_mutex;
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