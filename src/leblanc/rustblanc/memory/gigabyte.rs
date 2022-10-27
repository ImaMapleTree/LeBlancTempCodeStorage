use core::fmt::{Display, Formatter};
use num_format::{Locale, ToFormattedString};
use crate::leblanc::rustblanc::memory::kilobyte::Kilobyte;
use crate::leblanc::rustblanc::memory::megabyte::Megabyte;
use crate::leblanc::rustblanc::memory::MemoryUnit;
use crate::{mem_add, mem_cmp, mem_div, mem_mul, mem_sub};
use crate::leblanc::rustblanc::memory::byte::Byte;

#[derive(Copy, Clone, Debug)]
pub struct Gigabyte {
    n: usize
}

impl Gigabyte {
    pub const MEGABYTE_CONVERSION: usize = 1024;
    pub const KILOBYTE_CONVERSION: usize = 1048576;
    pub const BYTE_CONVERSION: usize = 1073741824;

    pub const fn new(n: usize) -> Self {
        Gigabyte { n: n * Self::BYTE_CONVERSION }
    }

    pub const fn to_byte(self) -> Byte {
        Byte::from(self)
    }

    pub const fn to_kb(self) -> Kilobyte {
        Kilobyte::from(self)
    }

    pub const fn to_mb(self) -> Megabyte {
        Megabyte::from(self)
    }

}

impl Display for Gigabyte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}GB", (self.n / Gigabyte::BYTE_CONVERSION).to_formatted_string(&Locale::en))
    }
}

impl const MemoryUnit for Gigabyte {
    fn usize(&self) -> usize {
        self.n
    }
}


impl const From<Megabyte> for Gigabyte {
    fn from(mb: Megabyte) -> Self {
        Gigabyte { n: mb.usize() }
    }
}

impl const From<Kilobyte> for Gigabyte {
    fn from(kb: Kilobyte) -> Self {
        Gigabyte { n: kb.usize() }
    }
}

impl const From<Byte> for Gigabyte {
    fn from(b: Byte) -> Self {
        Gigabyte { n: b.usize() }
    }
}

impl const From<usize> for Gigabyte {
    fn from(n: usize) -> Self {
        Gigabyte { n }
    }
}


mem_add!(Gigabyte, Gigabyte);
mem_add!(Gigabyte, Megabyte);
mem_add!(Gigabyte, Kilobyte);
mem_add!(Gigabyte, Byte);
mem_add!(Gigabyte, usize);

mem_sub!(Gigabyte, Gigabyte);
mem_sub!(Gigabyte, Megabyte);
mem_sub!(Gigabyte, Kilobyte);
mem_sub!(Gigabyte, Byte);
mem_sub!(Gigabyte, usize);

mem_mul!(Gigabyte, Gigabyte);
mem_mul!(Gigabyte, Megabyte);
mem_mul!(Gigabyte, Kilobyte);
mem_mul!(Gigabyte, Byte);
mem_mul!(Gigabyte, usize);

mem_div!(Gigabyte, Gigabyte);
mem_div!(Gigabyte, Megabyte);
mem_div!(Gigabyte, Kilobyte);
mem_div!(Gigabyte, Byte);
mem_div!(Gigabyte, usize);

mem_cmp!(Gigabyte, Gigabyte);
mem_cmp!(Gigabyte, Megabyte);
mem_cmp!(Gigabyte, Kilobyte);
mem_cmp!(Gigabyte, Byte);
mem_cmp!(Gigabyte, usize);