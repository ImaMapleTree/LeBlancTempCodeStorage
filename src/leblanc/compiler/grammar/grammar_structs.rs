use core::fmt::{Display, Formatter};
use std::collections::HashSet;
use crate::LeBlancType;

pub struct SyntaxError {

}

pub struct ScopeValue {
    pub scope: ScopeType,
    pub arg_types: Vec<LeBlancType>,
    pub types: Vec<LeBlancType>
}

impl ScopeValue {

}

impl PartialEq for ScopeValue {
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope
    }
}

pub enum ScopeType {
    Global,
    Class,
    Local(u64),
    NestedLocal(u64),
}

impl Display for ScopeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            ScopeType::Global => "Global",
            ScopeType::Class => "Class",
            ScopeType::Local(n) => format!("Local({})", n).as_str(),
            ScopeType::NestedLocal(n) => format!("NestedLocal({})", n).as_str(),
        };
        write!(f, "{}", s)
    }
}

impl PartialEq for ScopeType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ScopeType::Global => other.to_string() == self.to_string(),
            ScopeType::Class => other.to_string() == self.to_string(),
            ScopeType::Local(_) => other.to_string() == self.to_string(),
            ScopeType::NestedLocal(n) => {
                if let ScopeType::Local(on) = other {
                    return *on == *n
                }
                other.to_string() == self.to_string()
            }
        }
    }
}



pub struct ScopeSet {
    inner_map: HashSet<ScopeValue>
}