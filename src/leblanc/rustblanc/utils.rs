use std::num::ParseIntError;
use std::fmt::Write;
use crate::leblanc::rustblanc::hex::Hexadecimal;

pub fn decode_hex(s: &Hexadecimal) -> Result<Vec<u8>, ParseIntError> {
    let s = s.to_string();
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> Hexadecimal {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    Hexadecimal::from_string(s)
}

pub fn encode_hex_u16(bytes: &[u16]) -> Hexadecimal {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    Hexadecimal::from_string(s)
}

pub fn decode_hex_u16(s: &Hexadecimal) -> Result<Vec<u16>, ParseIntError> {
    let s = s.to_string();
    (0..s.len())
        .step_by(2)
        .map(|i| u16::from_str_radix(&s[i..i + 4], 16))
        .collect()
}