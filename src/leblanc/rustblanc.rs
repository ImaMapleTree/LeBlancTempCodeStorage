pub mod exception;
pub(crate) mod lib;
pub mod basic;
pub mod rust_override;
pub mod generic_data;
pub mod relationship;
pub mod utils;

pub trait Hexable {
    fn to_hex(&self, bytes: Option<u32>) -> String;

    fn from_hex(string: String) -> Self;
}