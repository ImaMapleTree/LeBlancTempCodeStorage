use core::fmt::{Display, Formatter};
use std::cmp::Ordering;
use num_format::{Locale, ToFormattedString};
use crate::leblanc::rustblanc::memory::byte::Byte;
use crate::leblanc::rustblanc::memory::kilobyte::Kilobyte;
use crate::leblanc::rustblanc::memory::MemoryUnit;
use crate::{mem_add, mem_cmp, mem_div, mem_mul, mem_sub};
use crate::leblanc::rustblanc::memory::gigabyte::Gigabyte;

#[derive(Copy, Clone, Debug)]
pub struct Megabyte {
    n: usize
}

impl Megabyte {
    pub const BYTE_CONVERSION: usize = 1048576;
    pub const KILOBYTE_CONVERSION: usize = 1024;

    pub const fn new(n: usize) -> Self {
        Megabyte { n: n * Megabyte::BYTE_CONVERSION }
    }

    pub const fn to_byte(self) -> Byte {
        Byte::from(self)
    }

    pub const fn to_kb(self) -> Kilobyte {
        Kilobyte::from(self)
    }
}

impl Display for Megabyte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} MB", (self.n / Self::BYTE_CONVERSION).to_formatted_string(&Locale::en))
    }
}

impl const MemoryUnit for Megabyte {
    fn usize(&self) -> usize {
        self.n
    }
}

impl const From<Gigabyte> for Megabyte {
    fn from(gb: Gigabyte) -> Self {
        Megabyte { n: gb.usize() }
    }
}

impl const From<Kilobyte> for Megabyte {
    fn from(kb: Kilobyte) -> Self {
        Megabyte { n: kb.usize() }
    }
}

impl const From<Byte> for Megabyte {
    fn from(b: Byte) -> Self {
        Megabyte { n: b.usize() }
    }
}

impl const From<usize> for Megabyte {
    fn from(n: usize) -> Self {
        Megabyte { n }
    }
}

mem_add!(Megabyte, Gigabyte);
mem_add!(Megabyte, Megabyte);
mem_add!(Megabyte, Kilobyte);
mem_add!(Megabyte, Byte);
mem_add!(Megabyte, usize);

mem_sub!(Megabyte, Gigabyte);
mem_sub!(Megabyte, Megabyte);
mem_sub!(Megabyte, Kilobyte);
mem_sub!(Megabyte, Byte);
mem_sub!(Megabyte, usize);

mem_mul!(Megabyte, Gigabyte);
mem_mul!(Megabyte, Megabyte);
mem_mul!(Megabyte, Kilobyte);
mem_mul!(Megabyte, Byte);
mem_mul!(Megabyte, usize);

mem_div!(Megabyte, Gigabyte);
mem_div!(Megabyte, Megabyte);
mem_div!(Megabyte, Kilobyte);
mem_div!(Megabyte, Byte);
mem_div!(Megabyte, usize);

mem_cmp!(Megabyte, Gigabyte);
mem_cmp!(Megabyte, Megabyte);
mem_cmp!(Megabyte, Kilobyte);
mem_cmp!(Megabyte, Byte);
mem_cmp!(Megabyte, usize);