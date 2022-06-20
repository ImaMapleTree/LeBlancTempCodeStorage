use std::collections::HashMap;
use std::num::ParseIntError;
use std::fmt::Write;
use std::hash::Hash;
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

#[derive(Clone)]
pub struct Timings {
    pub map: Option<HashMap<String, Timing>>
}

impl Timings {
    pub fn setup(&mut self) {
        self.map = Some(HashMap::new());
    }

    pub fn add_timing(&mut self, name: String, duration: f64) {
        let zero_timing = Timing{count: 0, time: 0.0};
        let timing = self.map.as_ref().unwrap().get(&name).unwrap_or(&zero_timing);
        let mut timing = timing.clone();
        timing.count += 1; timing.time += duration;
        self.map.as_mut().unwrap().insert(name.to_string(), timing);
    }

    pub fn print_timing(&self) {
        println!("{:?}", self.map);
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Timing {
    pub count: u32,
    pub time: f64
}