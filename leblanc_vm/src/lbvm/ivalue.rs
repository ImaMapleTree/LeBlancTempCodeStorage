use crate::arch::ArchSize;
use crate::lbvm::object::Object;

/// 4 byte value stored on the stack and read during run time
#[derive(Copy, Clone, Debug)]
pub(crate) struct IVal(pub(crate) ArchSize);

impl IVal {
    pub fn from_short(value: i16) -> Self {
        Self(value as u16 as ArchSize)
    }

    pub fn from_int(value: i32) -> Self {
        Self(value as u32 as ArchSize)
    }


    /*#[cfg(target_pointer_width = "64")]
    pub fn from_long(value: i64) -> (Self, Self) {
        (
            /*Self((value as u64 & 0xFFFFFFFF) as ArchSize),
            Self((value as u64 >> 32) as ArchSize),*/
            Self(value as u64)
        )
    }*/

    pub fn from_long(value: i64) -> (Self, Self) {
        (
            Self((value as u64 & 0xFFFFFFFF) as ArchSize),
            Self((value as u64 >> 32) as ArchSize),
        )
    }

    pub fn from_float(value: f32) -> Self {
        Self(value.to_bits() as ArchSize)
    }

    pub fn from_double(value: f64) -> (Self, Self) {
        Self::from_long(value.to_bits() as i64)
    }

    pub fn from_ref(value: Option<Object>) -> Self {
        Self::from_int(0)
    }

    pub fn as_int(&self) -> i32 {
        self.0 as u32 as i32
    }

    pub fn as_float(&self) -> f32 {
        f32::from_bits(self.0 as u32)
    }

    pub fn as_long(low: IVal, high: IVal) -> i64 {
        (low.0 as u32 as u64 + ((high.0 as u64) << 32)) as i64
    }

    pub fn as_double(low: IVal, high: IVal) -> f64 {
        f64::from_bits(low.0 as u32 as u64 + ((high.0 as u64) << 32))
    }
}