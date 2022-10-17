use core::str::FromStr;
use std::io;
use std::io::{BufRead, Stdin, stdout, Write};
use std::sync::Arc;
use smol_str::SmolStr;

use crate::leblanc::core::leblanc_object::{LeBlancObject, RustDataCast};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::class_type::{ClassMeta, leblanc_object_custom};
use crate::leblanc::core::native_types::rust_type::{RustObject, RustObjectBuilder, RustSubTrait, RustType};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::rustblanc::types::LBObject;

pub fn _stdin_read_(_self: Arc<Strawberry<LeBlancObject>>, _args: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line[0..line.len()-1].to_string().create_mutex()
}


pub fn _stdin_prompt_(_self: Arc<Strawberry<LeBlancObject>>, _args: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let locked = _args[0].underlying_pointer();
    let s: &SmolStr = locked.data.ref_data().unwrap();
    print!("{}", s);
    stdout().flush().unwrap();
    _stdin_read_(_self, _args)
}

pub fn _stdin_read_int_(_self: Arc<Strawberry<LeBlancObject>>, _args: Vec<LBObject>) -> Arc<Strawberry<LeBlancObject>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = line[0..line.len()-1].to_string();
    i32::from_str(&line).unwrap().create_mutex()
}
