use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::utils::{decode_hex, decode_hex_u16, encode_hex, encode_hex_u16};

impl Hexable for u16 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 2 {
            bytes = 2;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[2 - bytes..2])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        u16::from_be_bytes(<[u8; 2]>::try_from(bytes).unwrap())
    }
}

impl Hexable for i16 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 2 {
            bytes = 2;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[2 - bytes..2])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        i16::from_be_bytes(<[u8; 2]>::try_from(bytes).unwrap())
    }
}

impl Hexable for u32 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 4 {
            bytes = 4;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[4-bytes..4])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        u32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap())
    }
}

impl Hexable for i32 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 4 {
            bytes = 4;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[4-bytes..4])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        i32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap())
    }
}


impl Hexable for u64 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 8 {
            bytes = 8;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[8-bytes..8])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        u64::from_be_bytes(<[u8; 8]>::try_from(bytes).unwrap())
    }
}

impl Hexable for i64 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 8 {
            bytes = 8;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[8-bytes..8])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        i64::from_be_bytes(<[u8; 8]>::try_from(bytes).unwrap())
    }
}

impl Hexable for u128 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 16 {
            bytes = 16;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[16-bytes..16])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        u128::from_be_bytes(<[u8; 16]>::try_from(bytes).unwrap())
    }
}


impl Hexable for i128 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 16 {
            bytes = 16;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[16-bytes..16])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        i128::from_be_bytes(<[u8; 16]>::try_from(bytes).unwrap())
    }
}

impl Hexable for f32 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 4 {
            bytes = 4;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[4-bytes..4])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        f32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap())
    }
}

impl Hexable for f64 {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let mut bytes = bytes;
        if bytes > 8 {
            bytes = 8;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[8-bytes..8])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        f64::from_be_bytes(<[u8; 8]>::try_from(bytes).unwrap())
    }
}


impl Hexable for usize {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let usize_bytes = (usize::BITS/8) as usize;
        let mut bytes = bytes;
        if bytes as usize > usize_bytes {
            bytes = usize_bytes;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[usize_bytes-bytes..usize_bytes])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        usize::from_be_bytes(<[u8; 8]>::try_from(bytes).unwrap())
    }
}

impl Hexable for isize {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let usize_bytes = (isize::BITS/8) as usize;
        let mut bytes = bytes;
        if bytes as usize > usize_bytes {
            bytes = usize_bytes;
        }
        let bytes = bytes as usize;
        encode_hex(&self.to_be_bytes()[usize_bytes-bytes..usize_bytes])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        isize::from_be_bytes(<[u8; 8]>::try_from(bytes).unwrap())
    }
}

impl Hexable for bool {
    fn to_hex(&self, _bytes: usize) -> Hexadecimal {
        if *self {
            return Hexadecimal::new(vec!["01".to_string()])
        }
        Hexadecimal::new(vec!["00".to_string()])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        if hex.is_zero() {
           return false;
        }
        true
    }
}

impl Hexable for char {
    fn to_hex(&self, _bytes: usize) -> Hexadecimal {
        let u16_bytes: &mut [u16] = &mut [];
        char::encode_utf16(*self, u16_bytes);
        encode_hex_u16(u16_bytes)
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex_u16(hex).unwrap();
        let u16_bytes: [u16; 4] = <[u16; 4]>::try_from(bytes).unwrap();
        char::decode_utf16(u16_bytes).next().unwrap().unwrap()
    }
}

impl Hexable for String {
    fn to_hex(&self, _bytes: usize) -> Hexadecimal {
        encode_hex(self.as_bytes())
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let bytes = decode_hex(hex).unwrap();
        String::from_utf8_lossy(&bytes).to_string()
    }
}

pub trait OptionEquality<T> {
    fn ieq(&self, other: &Option<T>) -> bool;

    fn ieq_not_none(&self, other: &Option<T>) -> bool;
}

impl<T> OptionEquality<T> for Option<T>
where T: PartialEq
{
    fn ieq(&self, other: &Option<T>) -> bool {
        self.is_none() == other.is_none() ||
            (self.is_some() == other.is_some() && (self.as_ref().unwrap() == other.as_ref().unwrap()))
    }

    fn ieq_not_none(&self, other: &Option<T>) -> bool {
        (self.is_some() == other.is_some() && (self.as_ref().unwrap() == other.as_ref().unwrap()))
    }
}