use core::fmt::{Display, Formatter};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_, _internal_to_string_};
use crate::leblanc::core::internal::methods::internal_iterator::_internal_iterator_next;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::{base_clone_method, base_equals_method, base_expose_method, base_field_method, base_methods, base_to_string_method};
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::LeBlancType;

pub trait LeblancIterable {
    fn next(&mut self) -> LeBlancObject;
    fn has_next(&self) -> bool;
}

#[derive(PartialEq, Clone, Debug)]
pub struct LeblancIterator {
    iterator: Box<dyn LeblancIterable>
}

pub fn leblanc_object_iterator(leblanc_iterable: Box<dyn LeblancIterable>) -> LeBlancObject {
    let base_methods = iterator_methods();


    return LeBlancObject::new(
        LeBlancObjectData::Iterator(LeblancIterator::new(leblanc_iterable)),
        LeBlancType::Derived(DerivedType::Iterator),
        base_methods,
        HashMap::new(),
        VariableContext::empty(),
    )
}


impl LeblancIterator {
    pub fn new(iterator: Box<dyn LeblancIterable>) -> LeblancIterator {
        return LeblancIterator {
            iterator
        }
    }

    pub fn next(&mut self) -> LeBlancObject {
        return self.iterator.next();
    }

    pub fn has_next(&self) -> bool {
        return self.iterator.has_next();
    }
}

impl Display for LeblancIterator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "<Iterator>")
    }
}

pub fn iterator_methods() -> Arc<HashSet<Method>> {
    let mut hash_set = HashSet::new();
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_field_method(), _internal_field_));
    hash_set.insert( iterator_next_method());
    return Arc::new(hash_set);
}

pub fn iterator_next_method() -> Method {
    let method_store = MethodStore::new("next".to_string(), vec![]);
    return Method::new(
        method_store,
        _internal_iterator_next,
        BTreeSet::new()
    )
}

pub fn iterator_to_list() -> Method {
    let method_store = MethodStore::new("list".to_string(), vec![]);
    return Method::new(
        method_store,
        _internal_iterator_next,
        BTreeSet::new()
    )
}