use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Ord, Hash, PartialOrd, Debug, Copy, Clone)]
pub enum MethodTag {
    Addition,
    Subtraction,
    InPlaceAddition
}

impl MethodTag {
    pub fn singleton(&self) -> BTreeSet<MethodTag> {
        let mut set = BTreeSet::new();
        set.insert(*self);
        set
    }
}

impl Display for MethodTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}