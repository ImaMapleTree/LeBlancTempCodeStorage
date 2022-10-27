use core::fmt::Display;
use std::mem::{size_of};
use crate::leblanc::rustblanc::memory::byte::Byte;


pub mod megabyte;
pub mod byte;
pub mod kilobyte;
pub mod gigabyte;
pub mod heap;
pub mod pointer;



#[macro_export]
macro_rules! mem_add {
    ($_self:ident, $other:ident) => {
        impl const std::ops::Add<$other> for $_self {
            type Output = $_self;

            fn add(self, rhs: $other) -> Self::Output {
                $_self::from(self.usize() + rhs.usize())
            }
        }
    }
}

#[macro_export]
macro_rules! mem_sub {
    ($_self:ident, $other:ident) => {
        impl const std::ops::Sub<$other> for $_self {
            type Output = $_self;

            fn sub(self, rhs: $other) -> Self::Output {
                $_self::from(self.usize() - rhs.usize())
            }
        }
    }
}

#[macro_export]
macro_rules! mem_mul {
    ($_self:ident, $other:ident) => {
        impl const std::ops::Mul<$other> for $_self {
            type Output = $_self;

            fn mul(self, rhs: $other) -> Self::Output {
                $_self::from(self.usize() * rhs.usize())
            }
        }
    }
}

#[macro_export]
macro_rules! mem_div {
    ($_self:ident, $other:ident) => {
        impl const std::ops::Div<$other> for $_self {
            type Output = $_self;

            fn div(self, rhs: $other) -> Self::Output {
                $_self::from(self.usize() / rhs.usize())
            }
        }
    }
}

#[macro_export]
macro_rules! mem_cmp {
    ($_self:ident, $other:ident) => {
        impl PartialEq<$other> for $_self {
            fn eq(&self, other: &$other) -> bool {
                self.n == other.usize()
            }
        }
        impl PartialOrd<$other> for $_self {
            fn partial_cmp(&self, other: &$other) -> Option<std::cmp::Ordering> {
                self.n.partial_cmp(&other.usize())
            }
        }
    }
}

#[const_trait]
pub trait MemoryUnit: Display {
    fn usize(&self) -> usize;
}

#[const_trait]
pub trait MemoryFootprint: Sized {
    fn mem_size(&self) -> Byte;
}

impl const MemoryUnit for usize {
    fn usize(&self) -> usize {
        *self
    }
}

impl<T: Sized> const MemoryFootprint for T {
    fn mem_size(&self) -> Byte {
        Byte::from(size_of::<T>())
    }
}
