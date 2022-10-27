
use core::fmt::Debug;
use core::slice::Iter;

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use std::vec;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::include::lib::leblanc_colored::{Color, ColorBright, colorize};

#[derive(Clone, Hash, PartialEq, Eq, Default)]
pub struct Hexadecimal {
    bytes: Vec<String>
}

impl Hexadecimal {
    pub fn new(bytes: Vec<String>) -> Hexadecimal {
        Hexadecimal {
            bytes
        }
    }

    pub fn from_string(string: String) -> Hexadecimal {
        let mut hex_vec = vec![];
        for mut i in 0..string.len()/2 {
            hex_vec.push(string[(i*2)..(i*2)+2].to_string());
        }
        Hexadecimal::new(hex_vec)
    }

    pub fn append(&mut self, hex: &mut Hexadecimal) {
        self.bytes.append(&mut hex.bytes);
    }

    pub fn consume(&mut self, hex: Hexadecimal) {
        hex.bytes.into_iter().for_each(|h| self.bytes.push(h))
    }

    pub fn insert(&mut self, index: usize, hex: Hexadecimal) {
        let iter = hex.into_iter().rev();
        iter.for_each(|h| self.bytes.insert(index, h))
    }

    pub fn to_vec(self) -> Vec<String> {
        self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, String> {
        self.bytes.iter()
    }

    pub fn pop(&mut self, amount: usize) -> Hexadecimal {
        let mut new_hex = vec![];
        for _i in 0..amount {
            new_hex.insert(0, self.bytes.pop().unwrap())
        }

        Hexadecimal::new(new_hex)
    }

    pub fn is_zero(&self) -> bool {
        return self.bytes.iter().all(|b| b == "00")
    }

    pub fn leading_zeroes(&self) -> usize {
        self.bytes.iter().filter(|&b| b == "00").count()
    }

    pub fn strip_leading_zeroes(&mut self) {
        self.bytes = self.bytes.iter().cloned().filter(|b| b != "00").collect();
    }

    pub fn extend_to_length(&mut self, bytes: usize) {
        for _i in 0..bytes-self.bytes.len() {
            self.bytes.insert(0, "00".to_string());
        }
    }

    pub fn to_new_length(&self, bytes: usize) -> Hexadecimal {
        let mut new_bytes = self.clone();
        for _i in 0..bytes-self.bytes.len() {
            new_bytes.bytes.insert(0, "00".to_string());
        }
        new_bytes
    }

    pub fn bytes_at(&mut self, index: usize, size: usize) -> Hexadecimal {
        Hexadecimal::new(self.bytes[index..index+size].to_vec())
    }

    pub fn scrape(&mut self, amount: usize) -> Hexadecimal {
        let mut new_hex = vec![];
        for _i in 0..amount {
            new_hex.push(self.bytes.remove(0))
        }
        Hexadecimal::new(new_hex)
    }

    pub fn hex<T: Hexable>(mut self, other: T, bytes: usize) -> Hexadecimal {
        self.append(&mut other.to_hex(bytes)); self
    }

    pub fn to_hexable<T: Hexable>(&self) -> T {
        T::from_hex(self)
    }
}


impl Display for Hexadecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bytes.join(""))
    }
}

impl Debug for Hexadecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut s = String::new();
        let mut color = Color::Red;
        for byte in &self.bytes {
            s += &(" ".to_owned() + &colorize(byte.clone(), color));
            if color == Color::Red {
                color = Color::Bright(ColorBright::BrightYellow);
            } else {
                color = Color::Red;
            }

        }
        write!(f, "Hex({})", s.replacen(' ', "", 1))
    }
}

impl IntoIterator for Hexadecimal {
    type Item = String;
    type IntoIter = vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter()
    }
}

impl Add for Hexadecimal {
    type Output = Self;

    #[allow(clippy::redundant_clone)]
    fn add(mut self, rhs: Self) -> Self::Output {
        self.bytes.append(&mut rhs.bytes.clone());
        self
    }
}

impl AddAssign for Hexadecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.consume(rhs)
    }
}