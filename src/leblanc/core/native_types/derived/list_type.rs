use core::fmt::{Display, Formatter};
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::leblanc::rustblanc::strawberry::{Either, Strawberry};

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::LeBlancType;

#[derive(Clone, Debug)]
pub struct LeblancList {
    pub internal_vec: Vec<Strawberry<LeBlancObject>>
}

impl LeblancList {
    pub fn empty() -> LeblancList {
        return LeblancList {
            internal_vec: vec![]
        }
    }
}

pub fn leblanc_object_list_empty() -> LeBlancObject {
    let base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::List(LeblancList::empty()),
        LeBlancType::Derived(DerivedType::List),
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}

pub fn leblanc_object_list(list: LeblancList) -> LeBlancObject {
    let base_methods = base_methods();

    return LeBlancObject::new(
        LeBlancObjectData::List(list),
        LeBlancType::Derived(DerivedType::List),
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}

impl ToLeblanc for LeblancList {
    fn create(&self) -> LeBlancObject {
        return leblanc_object_list(self.clone());
    }
    fn create_mutex(&self) -> Strawberry<LeBlancObject> { return Strawberry::new(self.create()) }
}

impl Display for LeblancList {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}]", self.internal_vec.iter().map(|item| item.clone().call_name("to_string").loan().inquire().either().data.to_string()).collect::<Vec<String>>().join(", "))
    }
}

impl PartialEq for LeblancList {
    fn eq(&self, other: &Self) -> bool {
        if self.internal_vec.len() != other.internal_vec.len() { return false }
        for i in 0..self.internal_vec.len() {
            if self.internal_vec[i].bypass_loan().data != other.internal_vec[i].bypass_loan().data {
                return false;
            }
        }
        return true;
    }
}

impl PartialOrd for LeblancList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_len = self.internal_vec.len();
        let other_len = other.internal_vec.len();
        return if self_len == other_len {
            Some(Ordering::Equal)
        } else if self_len > other_len {
            Some(Ordering::Greater)
        } else if self_len < other_len {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}