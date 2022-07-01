use core::fmt::{Display, Formatter};
use std::cmp::Ordering;
use fxhash::{FxHashMap, FxHashSet};


use alloc::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;
use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_, _internal_to_string_};
use crate::leblanc::core::internal::methods::internal_list::{_internal_list_append_, _internal_list_iterate_};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;

use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::{base_clone_method, base_equals_method, base_expose_method, base_field_method, base_methods, base_to_string_method, ToLeblanc};
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::LeBlancType;

#[derive(Clone, Debug)]
pub struct LeblancList {
    pub internal_vec: Vec<Rc<RefCell<LeBlancObject>>>
}

impl LeblancList {
    pub fn empty() -> LeblancList {
        LeblancList {
            internal_vec: vec![]
        }
    }

    pub fn new(internal_vec: Vec<Rc<RefCell<LeBlancObject>>>) -> LeblancList {
        LeblancList {
            internal_vec
        }
    }
}

pub fn leblanc_object_list_empty() -> LeBlancObject {
    let base_methods = list_methods();

    LeBlancObject::new(
        LeBlancObjectData::List(LeblancList::empty()),
        LeBlancType::Derived(DerivedType::List),
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}

pub fn list_methods() -> Arc<FxHashSet<Method>> {
    let mut hash_set = FxHashSet::default();
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_field_method(), _internal_field_));
    hash_set.insert(list_iterate_method());
    hash_set.insert(list_append_method());
    Arc::new(hash_set)
}

pub fn list_iterate_method() -> Method {
    let method_store = MethodStore::new("iterate".to_string(), vec![]);
    Method::new(
        method_store,
        _internal_list_iterate_,
        BTreeSet::new()
    )
}

pub fn list_append_method() -> Method {
    let method_store = MethodStore::new("append".to_string(), vec![LeBlancArgument::default(LeBlancType::Flex, 0)]);
    Method::new(
        method_store,
        _internal_list_append_,
        BTreeSet::new()
    )
}

pub fn leblanc_object_list(list: LeblancList) -> LeBlancObject {
    let base_methods = list_methods();

    LeBlancObject::new(
        LeBlancObjectData::List(list),
        LeBlancType::Derived(DerivedType::List),
        base_methods,
        FxHashMap::default(),
        VariableContext::empty(),
    )
}

impl ToLeblanc for LeblancList {
    fn create(&self) -> LeBlancObject {
        leblanc_object_list(self.clone())
    }
    fn create_mutex(&self) -> Rc<RefCell<LeBlancObject>> { Rc::new(RefCell::new(self.create())) }
}

impl Display for LeblancList {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}]", self.internal_vec.iter().map(|item| item.clone().call_name("to_string").unwrap().borrow_mut().data.to_string()).collect::<Vec<String>>().join(", "))
    }
}

impl PartialEq for LeblancList {
    fn eq(&self, other: &Self) -> bool {
        if self.internal_vec.len() != other.internal_vec.len() { return false }
        for i in 0..self.internal_vec.len() {
            if self.internal_vec[i].borrow().data != other.internal_vec[i].borrow().data {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for LeblancList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_len = self.internal_vec.len();
        let other_len = other.internal_vec.len();
        if self_len == other_len {
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

impl RustDataCast<LeblancList> for LeBlancObjectData {
    fn clone_data(&self) -> Option<LeblancList> {
        match self {
            LeBlancObjectData::List(list) => Some(list.clone()),
            _ => None
        }
    }

    fn ref_data(&self) -> Option<&LeblancList> {
        match self {
            LeBlancObjectData::List(list) => Some(list),
            _ => None
        }
    }

    fn mut_data(&mut self) -> Option<&mut LeblancList> {
        match self {
            LeBlancObjectData::List(list) => Some(list),
            _ => None
        }
    }
}