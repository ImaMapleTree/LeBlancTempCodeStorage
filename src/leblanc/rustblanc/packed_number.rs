

use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;


#[derive(Copy, Clone, Debug)]
pub enum PackedRustType {
    PackedI16,
    PackedI32,
    PackedI64,
    PackedI128,
    PackedIsize,
    PackedU16,
    PackedU32,
    PackedU64,
    PackedU128,
    PackedUsize,
    PackedF32,
    PackedF64
}

#[derive(Clone, Debug)]
pub struct PackedNumber {
    number: Hexadecimal
}

impl PackedNumber {
    pub fn new<T: Hexable>(number: T) -> PackedNumber {
        PackedNumber {
            number: number.to_hex(128),
        }
    }

    pub fn as_i16(&self) -> i16 {
        self.number.to_hexable::<i16>()
    }

    pub fn as_i32(&self) -> i32 {
        if self.number.len() < 4 {
            return self.number.to_new_length(4).to_hexable::<i32>()
        }
        self.number.to_hexable::<i32>()
    }

    pub fn as_i64(&self) -> i64 {
        if self.number.len() < 8 {
            return self.number.to_new_length(8).to_hexable::<i64>()
        }
        self.number.to_hexable::<i64>()
    }

    pub fn as_i128(&self) -> i128 {
        if self.number.len() < 16 {
            return self.number.to_new_length(16).to_hexable::<i128>()
        }
        self.number.to_hexable::<i128>()
    }

    pub fn as_f32(&self) -> f32 {
        if self.number.len() < 4 {
            return self.number.to_new_length(4).to_hexable::<f32>()
        }
        self.number.to_hexable::<f32>()
    }

    pub fn as_f64(&self) -> f64 {
        if self.number.len() < 8 {
            return self.number.to_new_length(8).to_hexable::<f64>()
        }
        self.number.to_hexable::<f64>()
    }
}