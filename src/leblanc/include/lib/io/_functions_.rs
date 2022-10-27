use core::str::FromStr;
use std::io;
use std::io::{BufRead, stdout, Write};

use smol_str::SmolStr;

use crate::leblanc::core::leblanc_object::{RustDataCast};
use crate::leblanc::core::native_types::base_type::ToLeblanc;




use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};

pub fn _stdin_read_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line[0..line.len()-1].to_string().create_mutex()
}


pub fn _stdin_prompt_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    let locked = &_args[0];
    let s: &SmolStr = locked.data.ref_data().unwrap();
    print!("{}", s);
    stdout().flush().unwrap();
    _stdin_read_(_self, _args)
}

pub fn _stdin_read_int_(_self: LBObject, _args: LBObjArgs) -> LBObject {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = line[0..line.len()-1].to_string();
    i32::from_str(&line).unwrap().create_mutex()
}
