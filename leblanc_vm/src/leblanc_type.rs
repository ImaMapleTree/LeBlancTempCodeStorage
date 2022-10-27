use crate::lbvm::ivalue::IVal;
use crate::lbvm::object::Object;

pub enum LBType {
    Boolean,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Class(usize),
}

#[derive(Debug)]
pub enum LBValue {
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Ref(Option<Object>)
}

impl From<Option<Object>> for LBValue {
    fn from(val: Option<Object>) -> Self {
        Self::Ref(val)
    }
}

impl From<Object> for LBValue {
    fn from(val: Object) -> Self {
        Self::Ref(Some(val))
    }
}

impl From<i32> for LBValue {
    fn from(val: i32) -> Self {
        Self::Int(val)
    }
}

impl From<i64> for LBValue {
    fn from(val: i64) -> Self {
        Self::Long(val)
    }
}

impl From<f32> for LBValue {
    fn from(val: f32) -> Self {
        Self::Float(val)
    }
}

impl From<f64> for LBValue {
    fn from(val: f64) -> Self {
        Self::Double(val)
    }
}