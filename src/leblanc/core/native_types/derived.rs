use core::fmt::{Display, Formatter};

pub mod list_type;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Ord, PartialOrd, Debug)]
pub enum DerivedType {
    List
}

impl Display for DerivedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DerivedType::List => "list"
        };
        write!(f, "{}", s)
    }
}