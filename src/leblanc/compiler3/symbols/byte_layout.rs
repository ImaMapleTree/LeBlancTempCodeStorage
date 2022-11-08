use crate::leblanc::compiler3::symbols::byte_layout::LayoutKey::{Boundary, Inf, RepeatEnd, RepeatStart, U32};

pub static BYTE_BOUNDARY: u8 = 0;
pub static REPEAT_START: u8 = 254;
pub static REPEAT_END: u8 = 255;

pub static FUNC_LAYOUT: [LayoutKey; 9] = [
    Inf("Function Name"), Boundary, U32("Number of args"), U32("Number of locals"),
    RepeatStart, U32("Type"), Inf("Variable Name"), Boundary, RepeatEnd
];


pub enum LayoutKey {
    U8(&'static str),
    U16(&'static str),
    U32(&'static str),
    U64(&'static str),
    N(&'static str, u16),
    Inf(&'static str),
    Boundary,
    RepeatStart,
    RepeatEnd,
}
