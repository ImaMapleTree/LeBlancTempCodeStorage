use alloc::rc::Rc;
use core::fmt::{Display, Formatter};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use fxhash::{FxHashMap, FxHashSet};
use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_, _internal_to_string_};
use crate::leblanc::core::internal::methods::internal_promise::_internal_promise_consume_;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::{base_clone_method, base_equals_method, base_expose_method, base_field_method, base_to_string_method, ToLeblanc};
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::LeBlancType;

#[derive(Debug, Clone, Default)]
pub struct ArcLeblancPromise {
    pub inner: Arc<Strawberry<LeblancPromise>>
}

impl PartialEq for ArcLeblancPromise {
    fn eq(&self, other: &Self) -> bool {
        self.inner.lock().eq(&other.inner.lock())
    }
}

impl PartialOrd for ArcLeblancPromise {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.lock().partial_cmp(&other.inner.lock())
    }
}

impl ArcLeblancPromise {
    pub fn from(inner: Arc<Strawberry<LeblancPromise>>) -> ArcLeblancPromise {
        ArcLeblancPromise {
            inner
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LeblancPromise {
    pub result: Option<Arc<Strawberry<LeBlancObject>>>,
    pub complete: bool,
    pub consumed: bool,
}

impl PartialEq for LeblancPromise {
    fn eq(&self, other: &Self) -> bool {
        self.result.as_ref().unwrap().lock().eq(&other.result.as_ref().unwrap().lock())
    }
}

impl PartialOrd for LeblancPromise {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.eq(&other) {
            true => Some(Ordering::Equal),
            false => None
        }
    }
}

impl LeblancPromise {
    pub fn consume(&mut self) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>> {
        match self.complete {
            false => Err(LeblancError::new("PromiseNotFulfilledException".to_string(), "Attempted to consume a non-complete promise.".to_string(), vec![]).create_mutex()),
            true => {
                self.consumed = true;
                let res = Ok(self.result.as_ref().unwrap().lock().clone().to_mutex());
                self.result = None;
                res
            }
        }
    }

    pub fn to_leblanc_object(self) -> LeBlancObject {
        leblanc_object_promise(ArcLeblancPromise::from(Arc::new(Strawberry::new(self))))
    }
}

impl ToLeblanc for LeblancPromise {
    fn create(&self) -> LeBlancObject {
        leblanc_object_promise(ArcLeblancPromise::from(Arc::new(Strawberry::new(self.clone()))))
    }

    fn create_mutex(&self) -> Arc<Strawberry<LeBlancObject>> {
        self.create().to_mutex()
    }
}

impl ToLeblanc for Arc<Strawberry<LeblancPromise>> {
    fn create(&self) -> LeBlancObject {
        leblanc_object_promise(ArcLeblancPromise::from(self.clone()))
    }

    fn create_mutex(&self) -> Arc<Strawberry<LeBlancObject>> {
        self.create().to_mutex()
    }
}

impl ToLeblanc for ArcLeblancPromise {
    fn create(&self) -> LeBlancObject {
        leblanc_object_promise(self.clone())
    }

    fn create_mutex(&self) -> Arc<Strawberry<LeBlancObject>> {
        self.create().to_mutex()
    }
}

pub fn leblanc_object_promise(promise: ArcLeblancPromise) -> LeBlancObject {
    LeBlancObject::new(
        LeBlancObjectData::Promise(promise),
        LeBlancType::Promise,
        promise_methods(),
        Arc::new(Strawberry::new(FxHashMap::default())),
        VariableContext::empty(),
    )
}

pub fn promise_methods() -> Arc<FxHashSet<Method>> {
    let mut hash_set = FxHashSet::default();
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_field_method(), _internal_field_));
    hash_set.insert(promise_consume_method());
    Arc::new(hash_set)
}

pub fn promise_consume_method() -> Method {
    let method_store = MethodStore::new("consume".to_string(), vec![]);
    Method::new(
        method_store,
        _internal_promise_consume_,
        BTreeSet::new()
    )
}

impl RustDataCast<ArcLeblancPromise> for LeBlancObjectData {
    fn clone_data(&self) -> Option<ArcLeblancPromise> {
        match self {
            LeBlancObjectData::Promise(promise) => Some(promise.clone()),
            _ => None
        }
    }

    fn ref_data(&self) -> Option<&ArcLeblancPromise> {
        match self {
            LeBlancObjectData::Promise(promise) => Some(promise),
            _ => None
        }
    }

    fn mut_data(&mut self) -> Option<&mut ArcLeblancPromise> {
        match self {
            LeBlancObjectData::Promise(promise) => Some(promise),
            _ => None
        }
    }
}

impl Display for LeblancPromise {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = if self.consumed {
            String::from("ConsumedPromise")
        } else if self.complete {
            format!("CompletedPromise({:#?})", self.result.as_ref().unwrap().lock().data).replace('\n', "").replace("(    ", "(").replace(",)", ")")
        } else {
            String::from("Promise")
        };
        write!(f, "{}", s)
    }
}

impl Display for ArcLeblancPromise {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner.lock())
    }
}

