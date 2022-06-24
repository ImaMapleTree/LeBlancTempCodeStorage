use core::fmt::{Display, Formatter};

pub mod list_type;
pub mod iterator_type;
pub mod generator_type;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Ord, PartialOrd, Debug)]
pub enum DerivedType {
    List,
    Iterator
}

impl Display for DerivedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DerivedType::List => "list",
            DerivedType::Iterator => "iterator"
        };
        write!(f, "{}", s)
    }
}