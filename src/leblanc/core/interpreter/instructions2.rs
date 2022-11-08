use core::fmt::{Debug, Formatter};
use std::intrinsics::size_of;
use std::ops::Add;
use std::ptr;

use lazy_static::lazy_static;
use strum_macros::Display;
use crate::{lazystore, unhex_instruct};
use crate::leblanc::core::interpreter::instructions2::Instruction2::*;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::lazy_store::{Lazy, LazyStore, Strategy};

lazy_static! {
    static ref INSTRUCTIONS: LazyStore<Instruction2> = lazystore![NOREF(0, []), BADD_NATIVE(0, []), BSUB_NATIVE(0, []),
        LOAD_CONSTANT(1, [0]), LOAD_VARIABLE(1, [0]), STORE_VARIABLE(1, [0]),
        STORE_CINV(2, [0, 0]), LOAD_FUNCTION(1, [0]), CALL_BUILTIN(2, [0, 0]), CALL_NORMAL(2, [0, 0]),
        EQUALS(0, []), NOT_EQUALS(0, []), GREATER_EQUALS(0, []), GREATER(0, []), LESS_EQUALS(0, []), LESS(0, []),
        IF_EQUALS(1, [0]), IF_NOT_EQUALS(1, [0]), IF_GREATER_EQUALS(1, [0]), IF_GREATER(1, [0]), IF_LESS_EQUALS(1, [0]), IF_LESS(1, [0]),
        JUMP(1, [0]), JUMP_BACK(1, [0]), RETURN(1, [0])];
}


// Instruction Line Number
pub type ILN = u32;

#[allow(non_snake_case)]
#[derive(Display, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Instruction2 {
    NOREF(ILN, [u16; 0]),

    // Adds two native types
    BADD_NATIVE(ILN, [u16; 0]),

    // Subtracts two native types
    BSUB_NATIVE(ILN, [u16; 0]),

    // Loads Constant onto stack
    LOAD_CONSTANT(ILN, [u16; 1]),

    // Loads variable onto stack
    LOAD_VARIABLE(ILN, [u16; 1]),

    // Stores value on stack into variable
    STORE_VARIABLE(ILN, [u16; 1]),

    // Stores constant in variable (int a = 5)
    STORE_CINV(ILN, [u16; 2]),

    // Loads function to stack but does not call it
    LOAD_FUNCTION(ILN, [u16; 1]),

    // Calls function with no args
    CALL_BUILTIN(ILN, [u16; 2]),

    // Calls function u16 with u16 args
    CALL_NORMAL(ILN, [u16; 2]),

    EQUALS(ILN, [u16; 0]),

    NOT_EQUALS(ILN, [u16; 0]),

    GREATER_EQUALS(ILN, [u16; 0]),

    GREATER(ILN, [u16; 0]),

    LESS_EQUALS(ILN, [u16; 0]),

    LESS(ILN, [u16; 0]),

    // if a == b; b1 = jump
    IF_EQUALS(ILN, [u16; 1]),

    // if a != b; b1 = jump
    IF_NOT_EQUALS(ILN, [u16; 1]),

    // if a >= b; b1 = jump
    IF_GREATER_EQUALS(ILN, [u16; 1]),

    // if a > b; b1 = jump
    IF_GREATER(ILN, [u16; 1]),

    // if a <= b; b1 = jump
    IF_LESS_EQUALS(ILN, [u16; 1]),

    // if a < b; b1 = jump
    IF_LESS(ILN, [u16; 1]),


    // Jumps N instructions
    JUMP(ILN, [u16; 1]),

    // Jumps backward N instructions
    JUMP_BACK(ILN, [u16; 1]),

    // Returns with u16 values
    RETURN(ILN, [u16; 1]),
}

impl Default for Instruction2 {
    fn default() -> Self {
        NOREF(0, [])
    }
}

impl Instruction2 {
    pub fn supple_bytes(inum: u16) -> u8 {
        INSTRUCTIONS.get(inum as usize).unwrap().line() as u8
    }

    pub fn get_inum(&self) -> u16 {
        INSTRUCTIONS.index(self, Strategy::LAZY).expect("Instruction not in static representation") as u16
    }

    pub fn index(instruct_name: &str) -> Option<usize> {
        INSTRUCTIONS.iter().position(|i| i.to_string() == instruct_name)
    }

    pub fn line(&self) -> u32 {
        match self {
            NOREF(ln, _) => *ln,
            BADD_NATIVE(ln, _) => *ln,
            BSUB_NATIVE(ln, _) => *ln,
            LOAD_CONSTANT(ln, _) => *ln,
            LOAD_VARIABLE(ln, _) => *ln,
            STORE_VARIABLE(ln, _) => *ln,
            STORE_CINV(ln, _) => *ln,
            LOAD_FUNCTION(ln, _) => *ln,
            CALL_BUILTIN(ln, _) => *ln,
            CALL_NORMAL(ln, _) => *ln,
            RETURN(ln, _) => *ln,
            EQUALS(ln, _) => *ln,
            NOT_EQUALS(ln, _) => *ln,
            GREATER_EQUALS(ln, _) => *ln,
            GREATER(ln, _) => *ln,
            LESS_EQUALS(ln, _) => *ln,
            LESS(ln, _) => *ln,
            IF_EQUALS(ln, _) => *ln,
            IF_NOT_EQUALS(ln, _) => *ln,
            IF_GREATER_EQUALS(ln, _) => *ln,
            IF_GREATER(ln, _) => *ln,
            IF_LESS_EQUALS(ln, _) => *ln,
            IF_LESS(ln, _) => *ln,
            JUMP(ln, _) => *ln,
            JUMP_BACK(ln, _) => *ln
        }
    }

    pub fn bytes2(&self, length: usize) -> &[u16] {
        unsafe {
            &*ptr::slice_from_raw_parts(((self as *const Instruction2 as *mut Instruction2).cast::<u16>()).byte_add(2), length)
        }
    }

    pub fn byte(&self, index: usize) -> u16 {
        unsafe {
            *(self as *const Instruction2 as *const u16).byte_add(2).add(index)
        }
    }

    pub fn bytes(&self) -> &[u16] {
        match self {
            NOREF(_, bytes) => bytes,
            BADD_NATIVE(_, bytes) => bytes,
            BSUB_NATIVE(_, bytes) => bytes,
            LOAD_CONSTANT(_, bytes) => bytes,
            LOAD_VARIABLE(_, bytes) => bytes,
            STORE_VARIABLE(_, bytes) => bytes,
            STORE_CINV(_, bytes) => bytes,
            LOAD_FUNCTION(_, bytes) => bytes,
            CALL_BUILTIN(_, bytes) => bytes,
            CALL_NORMAL(_, bytes) => bytes,
            EQUALS(_, bytes) => bytes,
            NOT_EQUALS(_, bytes) => bytes,
            GREATER_EQUALS(_, bytes) => bytes,
            GREATER(_, bytes) => bytes,
            LESS_EQUALS(_, bytes) => bytes,
            LESS(_, bytes) => bytes,
            IF_EQUALS(_, bytes) => bytes,
            IF_NOT_EQUALS(_, bytes) => bytes,
            IF_GREATER_EQUALS(_, bytes) => bytes,
            IF_GREATER(_, bytes) => bytes,
            IF_LESS_EQUALS(_, bytes) => bytes,
            IF_LESS(_, bytes) => bytes,
            RETURN(_, bytes) => bytes,
            JUMP(_, bytes) => bytes,
            JUMP_BACK(_, bytes) => bytes
        }
    }

    pub fn tuple_hex(&self) -> (Hexadecimal, Hexadecimal) {
        let ihex = self.get_inum().to_hex(2);
        let mut ahex = Hexadecimal::default();
        self.bytes().iter().for_each(|b| ahex.append(&mut b.to_hex(2)));
        (ihex, ahex)
    }
}

impl Lazy for Instruction2 {
    fn lazy() -> Strategy {
        Strategy::LAZY
    }

    fn standard() -> Strategy {
        Strategy::STANDARD
    }

    fn rust() -> Strategy {
        Strategy::RUST
    }

    fn scan(&self, other: &Self, strategy: Strategy) -> bool {
        match strategy {
            Strategy::LAZY => {
                self.to_string() == other.to_string()
            }
            _ => {
                self == other
            }
        }
    }
}

impl From<(Hexadecimal, u32)> for Instruction2 {
    fn from(data: (Hexadecimal, u32)) -> Self {
        let (mut hex, line) = data;
        let inum = hex.scrape(2).to_hexable::<u16>();
        Instruction2::from((inum, hex, line))
    }
}

impl From<(Hexadecimal, Hexadecimal, u32)> for Instruction2 {
    fn from(data: (Hexadecimal, Hexadecimal, u32)) -> Self {
        let (ihex, ahex, line) = data;
        let inum = ihex.to_hexable::<u16>();
        Instruction2::from((inum, ahex, line))
    }
}

impl From<(u16, Hexadecimal, u32)> for Instruction2 {
    fn from(data: (u16, Hexadecimal, u32)) -> Self {
        let (inum, mut ahex, line) = data;
        match inum {
            0 => unhex_instruct!(line, NOREF),
            1 => unhex_instruct!(line, BADD_NATIVE),
            2 => unhex_instruct!(line, BSUB_NATIVE),
            3 => unhex_instruct!(line, LOAD_CONSTANT, ahex),
            4 => LOAD_VARIABLE(line, [scrape_arg(&mut ahex)]),
            5 => STORE_VARIABLE(line, [scrape_arg(&mut ahex)]),
            6 => STORE_CINV(line, [scrape_arg(&mut ahex), scrape_arg(&mut ahex)]),
            7 => unhex_instruct!(line, LOAD_FUNCTION, ahex),
            8 => CALL_BUILTIN(line, [scrape_arg(&mut ahex), scrape_arg(&mut ahex)]),
            9 => CALL_NORMAL(line, [scrape_arg(&mut ahex), scrape_arg(&mut ahex)]),
            10 => EQUALS(line, []),
            11 => NOT_EQUALS(line, []),
            12 => GREATER_EQUALS(line, []),
            13 => GREATER(line, []),
            14 => LESS_EQUALS(line, []),
            15 => LESS(line, []),
            16 => IF_EQUALS(line, [scrape_arg(&mut ahex)]),
            17 => IF_NOT_EQUALS(line, [scrape_arg(&mut ahex)]),
            18 => IF_GREATER_EQUALS(line, [scrape_arg(&mut ahex)]),
            19 => IF_GREATER(line, [scrape_arg(&mut ahex)]),
            20 => IF_LESS_EQUALS(line, [scrape_arg(&mut ahex)]),
            21 => IF_LESS(line, [scrape_arg(&mut ahex)]),
            22 => unhex_instruct!(line, JUMP, ahex),
            23 => unhex_instruct!(line, JUMP_BACK, ahex),
            24 => RETURN(line, [scrape_arg(&mut ahex)]),
            _ => panic!("Unsupported Instruction")
        }
    }
}

/*impl Hexable for Instruction2 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let (mut ihex, mut ahex) = self.tuple_hex();
        ihex.append(&mut ahex); ihex
    }
}*/

fn scrape_arg(hex: &mut Hexadecimal) -> u16 {
    hex.scrape(2).to_hexable()
}


pub fn fmt_bytes(bytes: &[u16]) -> String {
    let s = format!("{:?}", bytes);
    if s == "[]" {
        String::from(" ")
    } else {
        s.replace(['[', ']', ','], "")
    }
}

impl Debug for Instruction2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            NOREF(..) => write!(f, "NOREF"),
            BADD_NATIVE(_, bytes) => write!(f, "BADD_NATIVE\t\t\t{}", fmt_bytes(bytes)),
            BSUB_NATIVE(_, bytes) => write!(f, "BSUB_NATIVE\t\t\t{}", fmt_bytes(bytes)),
            LOAD_CONSTANT(_, bytes) => write!(f, "LOAD_CONSTANT\t\t{}", fmt_bytes(bytes)),
            LOAD_VARIABLE(_, bytes) => write!(f, "LOAD_VARIABLE\t\t{}", fmt_bytes(bytes)),
            STORE_VARIABLE(_, bytes) => write!(f, "STORE_VARIABLE\t\t{}", fmt_bytes(bytes)),
            STORE_CINV(_, bytes) => write!(f, "STORE_CINV\t\t\t{}", fmt_bytes(bytes)),
            LOAD_FUNCTION(_, bytes) => write!(f, "LOAD_FUNCTION\t\t{}", fmt_bytes(bytes)),
            CALL_BUILTIN(_, bytes) => write!(f, "CALL_BUILTIN\t\t{}", fmt_bytes(bytes)),
            CALL_NORMAL(_, bytes) => write!(f, "CALL_NORMAL\t\t\t{}", fmt_bytes(bytes)),
            EQUALS(_, bytes) => write!(f, "EQUALS\t\t\t{:?}", fmt_bytes(bytes)),
            NOT_EQUALS(_, bytes) => write!(f, "NOT_EQUALS\t\t\t{}", fmt_bytes(bytes)),
            GREATER_EQUALS(_, bytes) => write!(f, "GREATER_EQUALS\t\t\t{}", fmt_bytes(bytes)),
            GREATER(_, bytes) => write!(f, "GREATER\t\t\t{:?}", fmt_bytes(bytes)),
            LESS_EQUALS(_, bytes) => write!(f, "LESS_EQUALS\t\t\t{}", fmt_bytes(bytes)),
            LESS(_, bytes) => write!(f, "LESS\t\t\t{:?}", fmt_bytes(bytes)),
            IF_EQUALS(_, bytes) => write!(f, "IF_EQUALS\t\t\t{}", fmt_bytes(bytes)),
            IF_NOT_EQUALS(_, bytes) => write!(f, "IF_NOT_EQUALS\t\t\t{}", fmt_bytes(bytes)),
            IF_GREATER_EQUALS(_, bytes) => write!(f, "IF_GREATER_EQUALS\t\t\t{}", fmt_bytes(bytes)),
            IF_GREATER(_, bytes) => write!(f, "IF_GREATER\t\t\t{}", fmt_bytes(bytes)),
            IF_LESS_EQUALS(_, bytes) => write!(f, "IF_LESS_EQUALS\t\t{}", fmt_bytes(bytes)),
            IF_LESS(_, bytes) => write!(f, "IF_LESS\t\t\t{}", fmt_bytes(bytes)),
            JUMP(_, bytes) => write!(f, "JUMP\t\t\t{}", fmt_bytes(bytes)),
            JUMP_BACK(_, bytes) => write!(f, "JUMP_BACK\t\t\t{}", fmt_bytes(bytes)),
            RETURN(_, bytes) => write!(f, "RETURN\t\t\t\t{}", fmt_bytes(bytes)),
        }
    }
}