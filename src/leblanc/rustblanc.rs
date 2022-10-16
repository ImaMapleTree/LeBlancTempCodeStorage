use crate::leblanc::rustblanc::hex::Hexadecimal;


pub mod rust_override;
pub(crate) mod utils;
pub mod hex;
pub(crate) mod copystring;
pub(crate) mod exception;
pub(crate) mod component_map;
pub mod strawberry;
pub mod bridge;
pub mod types;
pub(crate) mod lazy_store;
pub mod path;
pub mod lb_file;
pub mod outcome;
pub mod macros;


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
