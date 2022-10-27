use core::fmt::{Display, Formatter};
use num_format::{Locale, ToFormattedString};
use crate::leblanc::rustblanc::memory::byte::Byte;
use crate::leblanc::rustblanc::memory::megabyte::Megabyte;
use crate::leblanc::rustblanc::memory::MemoryUnit;
use crate::{mem_add, mem_cmp, mem_div, mem_mul, mem_sub};
use crate::leblanc::rustblanc::memory::gigabyte::Gigabyte;

#[derive(Copy, Clone, Debug)]
pub struct Kilobyte {
    n: usize
}

impl Kilobyte {
    pub const MEGABYTE_CONVERSION: usize = 1024;
    pub const BYTE_CONVERSION: usize = 1024;

    pub const fn new(n: usize) -> Self {
        Kilobyte { n: n * Self::BYTE_CONVERSION }
    }

    pub const fn to_byte(self) -> Byte {
        Byte::from(self)
    }

    pub const fn to_mb(self) -> Megabyte {
        Megabyte::from(self)
    }
}


impl Display for Kilobyte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} KB", (self.n / Self::BYTE_CONVERSION).to_formatted_string(&Locale::en))
    }
}

impl const MemoryUnit for Kilobyte {
    fn usize(&self) -> usize {
        self.n
    }
}

impl const From<Gigabyte> for Kilobyte {
    fn from(gb: Gigabyte) -> Self {
        Kilobyte { n: gb.usize() }
    }
}

impl const From<Megabyte> for Kilobyte {
    fn from(mb: Megabyte) -> Self {
        Kilobyte { n: mb.usize() }
    }
}

impl const From<Byte> for Kilobyte {
    fn from(b: Byte) -> Self {
        Kilobyte { n: b.usize() }
    }
}

impl const From<usize> for Kilobyte {
    fn from(n: usize) -> Self {
        Kilobyte { n }
    }
}

mem_add!(Kilobyte, Gigabyte);
mem_add!(Kilobyte, Megabyte);
mem_add!(Kilobyte, Kilobyte);
mem_add!(Kilobyte, Byte);
mem_add!(Kilobyte, usize);

mem_sub!(Kilobyte, Gigabyte);
mem_sub!(Kilobyte, Megabyte);
mem_sub!(Kilobyte, Kilobyte);
mem_sub!(Kilobyte, Byte);
mem_sub!(Kilobyte, usize);

mem_mul!(Kilobyte, Gigabyte);
mem_mul!(Kilobyte, Megabyte);
mem_mul!(Kilobyte, Kilobyte);
mem_mul!(Kilobyte, Byte);
mem_mul!(Kilobyte, usize);

mem_div!(Kilobyte, Gigabyte);
mem_div!(Kilobyte, Megabyte);
mem_div!(Kilobyte, Kilobyte);
mem_div!(Kilobyte, Byte);
mem_div!(Kilobyte, usize);

mem_cmp!(Kilobyte, Gigabyte);
mem_cmp!(Kilobyte, Megabyte);
mem_cmp!(Kilobyte, Kilobyte);
mem_cmp!(Kilobyte, Byte);
mem_cmp!(Kilobyte, usize);
