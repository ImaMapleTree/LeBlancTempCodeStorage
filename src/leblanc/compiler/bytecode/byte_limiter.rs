use core::slice::{Iter, IterMut};
use crate::leblanc::rustblanc::hex::Hexadecimal;

#[derive(Debug, Copy, Clone)]
pub enum ByteLimit {
    Limited(u64),
    Undefined
}

#[derive(Debug, Clone)]
pub struct ByteRestriction {
    limit: ByteLimit,
    repeated: bool,
    bytes: Hexadecimal,
    segments: Vec<Hexadecimal>
}

impl ByteRestriction {
    pub fn new(limit: ByteLimit, repeated: bool) -> ByteRestriction {
        ByteRestriction {
            limit,
            repeated,
            bytes: Hexadecimal::empty(),
            segments: vec![]
        }
    }
    pub fn once(limit: ByteLimit) -> ByteRestriction {
        ByteRestriction::new(limit, false)
    }
    pub fn repeated(limit: ByteLimit) -> ByteRestriction {
        ByteRestriction::new(limit, true)
    }

    pub fn add_bytes(&mut self, mut bytes: Hexadecimal) -> Result<usize, ()> {
        let size_comparison = if !self.repeated {
            (self.bytes.len() + bytes.len()) as u64
        } else {
            bytes.len() as u64
        };
        if let ByteLimit::Limited(limit) = self.limit {
            if size_comparison > limit {
                if size_comparison - bytes.leading_zeroes() as u64 > limit {
                    return Err(());
                }
                bytes.strip_leading_zeroes();
                bytes.extend_to_length((limit - self.bytes.len() as u64) as usize);
            }
        }

        if self.repeated {
            let length = bytes.len();
            self.segments.push(bytes);
            Ok(length)
        } else {
            self.bytes.append(&mut bytes);
            Ok(self.bytes.len())
        }
    }

    pub fn consume_bytes(&mut self, mut bytes: Hexadecimal) -> Result<usize, ()> {
        let size_comparison = if !self.repeated {
            (self.bytes.len() + bytes.len()) as u64
        } else {
            bytes.len() as u64
        };
        if let ByteLimit::Limited(limit) = self.limit {
            if size_comparison > limit {
                if size_comparison - bytes.leading_zeroes() as u64 > limit {
                    return Err(());
                }
                bytes.strip_leading_zeroes();
                bytes.extend_to_length((limit - self.bytes.len() as u64) as usize);
            }
        }

        if self.repeated {
            let length = bytes.len();
            self.segments.push(bytes);
            Ok(length)
        } else {
            self.bytes.consume(bytes);
            Ok(self.bytes.len())
        }
    }

    pub fn join(&self, data: &ByteRestriction) -> Hexadecimal {
        let mut hex = Hexadecimal::empty();
        for i in 0..self.segments.len() {
            hex.append(&mut self.segments[i].clone());
            hex.append(&mut data.segments[i].clone());
        }
        hex
    }

    pub fn join_uncloned(&mut self, data: &mut ByteRestriction) -> Hexadecimal {
        let mut hex = Hexadecimal::empty();
        for i in 0..self.segments.len() {
            hex.append(&mut self.segments[i]);
            hex.append(&mut data.segments[i]);
        }
        hex
    }

    pub fn join_thrice(&self, second: &ByteRestriction, third: &ByteRestriction) -> Hexadecimal {
        let mut hex = Hexadecimal::empty();
        for i in 0..self.segments.len() {
            hex.append(&mut self.segments[i].clone());
            hex.append(&mut second.segments[i].clone());
            hex.append(&mut third.segments[i].clone());
        }
        hex
    }

    pub fn bytes(&self) -> Hexadecimal {
        if self.repeated {
            let mut hex = Hexadecimal::empty();
            self.segments.iter().for_each(|seg| hex += seg.clone());
            return hex;
        }
        self.bytes.clone()
    }

    pub fn unpack(&self) -> Option<u64> {
        if let ByteLimit::Limited(amount ) = self.limit {
            return Some(amount);
        }
        None
    }

    pub fn iter(&self) -> Option<Iter<'_, Hexadecimal>> {
        if !self.repeated {
            return None;
        }
       Some(self.segments.iter())
    }

    pub fn iter_mut(&mut self) -> Option<IterMut<'_, Hexadecimal>> {
        if !self.repeated {
            return None;
        }
        Some(self.segments.iter_mut())
    }

    pub fn segments(&mut self) -> Option<Vec<Hexadecimal>> {
        if !self.repeated {
            return None;
        }
        Some(self.segments.clone())
    }

    pub fn pop(&mut self) -> Option<Hexadecimal> {
        if !self.repeated {
            return None;
        }
        self.segments.pop()
    }

    pub fn remove(&mut self, index: usize) -> Option<Hexadecimal> {
        if !self.repeated {
            return None;
        }
        Some(self.segments.remove(index))
    }

}