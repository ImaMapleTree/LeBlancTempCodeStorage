use core::fmt::{Display, Formatter};
use crate::leblanc::rustblanc::copystring::CopyString;
use crate::leblanc::core::native_types::LeBlancType;

pub mod list_type;
pub mod iterator_type;
pub mod generator_type;
pub mod slice_type;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Ord, PartialOrd, Debug)]
pub enum DerivedType {
    List,
    TypedList(CopyString),
    Iterator,
    Slice
}

impl Display for DerivedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DerivedType::List => "List",
            DerivedType::TypedList(_) => "List",
            DerivedType::Iterator => "iterator",
            DerivedType::Slice => "slice"
        };
        write!(f, "{}", s)
    }
}