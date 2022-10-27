use core::fmt::{Display, Formatter};
use num_format::{Locale, ToFormattedString};
use crate::leblanc::rustblanc::memory::kilobyte::Kilobyte;
use crate::leblanc::rustblanc::memory::megabyte::Megabyte;
use crate::leblanc::rustblanc::memory::MemoryUnit;
use crate::{mem_add, mem_cmp, mem_div, mem_mul, mem_sub};
use crate::leblanc::rustblanc::memory::gigabyte::Gigabyte;

#[derive(Copy, Clone, Debug)]
pub struct Byte {
    n: usize
}

impl Byte {
    pub const MEGABYTE_CONVERSION: usize = 1048576;
    pub const KILOBYTE_CONVERSION: usize = 1024;

    pub const fn new(n: usize) -> Self {
        Byte { n }
    }

    pub const fn to_kb(self) -> Kilobyte {
        Kilobyte::from(self)
    }

    pub const fn to_mb(self) -> Megabyte {
        Megabyte::from(self)
    }

}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} B", self.n.to_formatted_string(&Locale::en))
    }
}

impl const MemoryUnit for Byte {
    fn usize(&self) -> usize {
        self.n
    }
}

impl const From<Gigabyte> for Byte {
    fn from(gb: Gigabyte) -> Self {
        Byte { n: gb.usize() }
    }
}

impl const From<Megabyte> for Byte {
    fn from(mb: Megabyte) -> Self {
        Byte { n: mb.usize() }
    }
}

impl const From<Kilobyte> for Byte {
    fn from(kb: Kilobyte) -> Self {
        Byte { n: kb.usize() }
    }
}

impl const From<usize> for Byte {
    fn from(n: usize) -> Self {
        Byte { n }
    }
}


mem_add!(Byte, Gigabyte);
mem_add!(Byte, Megabyte);
mem_add!(Byte, Kilobyte);
mem_add!(Byte, Byte);
mem_add!(Byte, usize);

mem_sub!(Byte, Gigabyte);
mem_sub!(Byte, Megabyte);
mem_sub!(Byte, Kilobyte);
mem_sub!(Byte, Byte);
mem_sub!(Byte, usize);

mem_mul!(Byte, Gigabyte);
mem_mul!(Byte, Megabyte);
mem_mul!(Byte, Kilobyte);
mem_mul!(Byte, Byte);
mem_mul!(Byte, usize);

mem_div!(Byte, Gigabyte);
mem_div!(Byte, Megabyte);
mem_div!(Byte, Kilobyte);
mem_div!(Byte, Byte);
mem_div!(Byte, usize);

mem_cmp!(Byte, Gigabyte);
mem_cmp!(Byte, Megabyte);
mem_cmp!(Byte, Kilobyte);
mem_cmp!(Byte, Byte);
mem_cmp!(Byte, usize);