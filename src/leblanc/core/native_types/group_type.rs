use alloc::rc::Rc;
use core::fmt::{Debug, Display, Formatter};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::future::Future;
use std::mem::take;
use std::process::Output;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use async_std::task::JoinHandle;
use futures::executor::block_on;
use futures::future::join_all;
use fxhash::{FxHashMap, FxHashSet};
use crate::leblanc::core::internal::methods::internal_class::{_internal_expose_, _internal_field_, _internal_to_string_};
use crate::leblanc::core::internal::methods::internal_group::{_internal_group_apply_, _internal_group_pipe_, _internal_group_pipe_async_};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::{base_clone_method, base_equals_method, base_expose_method, base_field_method, base_to_string_method, ToLeblanc};
use crate::leblanc::core::native_types::promise_type::{LeblancPromise};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::LBObject;

#[derive(Clone, Debug, Default)]
pub struct PromiseCell {
    echo: LeBlancObject,
    promise: Arc<Strawberry<LeblancPromise>>,
}

impl PartialEq for PromiseCell {
    fn eq(&self, other: &Self) -> bool {
        if self.echo != other.echo { return false }
        self.promise.read().eq(&other.promise.read())
    }
}

impl PromiseCell {
    pub fn new(echo: LeBlancObject, promise: Arc<Strawberry<LeblancPromise>>) -> PromiseCell {
        PromiseCell {
            echo,
            promise
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LeblancGroup {
    promises: Vec<Arc<Strawberry<PromiseCell>>>,
    strict_type: Option<LeBlancType>,
}

impl PartialEq for LeblancGroup {
    fn eq(&self, other: &Self) -> bool {
        let self_length = self.promises.len();
        if self_length!= other.promises.len() { return false }
        for i in 0..self_length {
            if !self.promises[i].read().eq(&other.promises[i].read()) {return false}
        }
        return true;
    }
}

impl PartialOrd for LeblancGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.eq(&other) {
            true => Some(Ordering::Equal),
            false => None
        }
    }
}

impl LeblancGroup {
    pub fn promise(&mut self, returnable: Arc<Strawberry<LeBlancObject>>) -> Arc<Strawberry<LeblancPromise>> {
        let promise = Arc::new(Strawberry::new(LeblancPromise::default()));
        let cell = PromiseCell::new((*returnable).read().clone(), promise.clone());
        self.promises.push(Arc::new(Strawberry::new(cell)));
        promise
    }

    pub fn apply(&mut self, function: Arc<Strawberry<LeBlancObject>>, other_args: Vec<LBObject>) {
        self.promises.iter_mut().for_each(|prom| {
            let mut mutex = prom.read();
            let consumed = mutex.promise.read().consumed;
            if !consumed {
                mutex.promise.read().result = {
                    let mut args = other_args.to_vec();
                    args.insert(0, take(&mut mutex.echo).to_mutex());
                    let result = function.read().data.get_mut_inner_method().unwrap().clone().run(function.clone(), args);
                    Some(result)
                };
                mutex.promise.read().complete = true
            }
        })
    }

    pub fn pipe(&mut self, args: Vec<LBObject>) {
        self.promises.iter_mut().for_each(|prom| {
            let mut mutex = prom.read();
            let consumed = mutex.promise.read().consumed;
            let truth = !consumed && if self.strict_type.is_some() { self.strict_type.unwrap() == mutex.echo.typing } else { true };
            if truth {
                mutex.promise.read().result = {
                    let result = mutex.echo.data.get_mut_inner_method().unwrap().run(LeBlancObject::unsafe_null(), args.clone());
                    Some(result)
                };
                mutex.promise.read().complete = true
            }
        })
    }

    pub fn pipe_async(&mut self, args: Vec<LBObject>) {
        let mut consumers = vec![];
        let mut futures_functions = vec![];
        self.promises.iter_mut().for_each(|prom| {
            let consumed = prom.read().promise.read().consumed;
            let truth = !consumed && if self.strict_type.is_some() { self.strict_type.unwrap() == prom.read().echo.typing } else { true };
            if truth {
                futures_functions.push(prom.read().echo.data.get_mut_inner_method().unwrap().clone());
                consumers.push(prom);
            }
        });
        //let args = args.to_vec();

        let mut nargs = args.to_vec();
        let real_futures: Vec<JoinHandle<Arc<Strawberry<LeBlancObject>>>> = futures_functions.into_iter().map(|a| a.leblanc_handle.full_clone()).map(|mut f| {
            let nargs_clone = nargs.clone();
            async_std::task::spawn(async move {
                f.execute_async(nargs_clone).await}
        )}).collect();
        //map(async_std::task::spawn).collect();
        let mut tasks: Vec<Arc<Strawberry<LeBlancObject>>> = block_on(async {join_all(real_futures)
            .await
            .into_iter()
            .collect()});
        while !consumers.is_empty() {
            let prom = consumers.pop().unwrap();
            let mutex = prom.read();
            let result = tasks.pop().unwrap();
            mutex.promise.read().result = Some(result);
            mutex.promise.read().complete = true;
        }
        println!("N");
    }
}

async fn join_parallel<T: Send + 'static>(
    futs: impl IntoIterator<Item = impl Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let tasks: Vec<_> = futs.into_iter().map(tokio::spawn).collect();
    // unwrap the Result because it is introduced by tokio::spawn()
    // and isn't something our caller can handle
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

unsafe impl Send for LeBlancObject {}

unsafe impl Send for Method {}

unsafe impl Send for LeblancHandle {}

impl RustDataCast<LeblancGroup> for LeBlancObjectData {
    fn clone_data(&self) -> Option<LeblancGroup> {
        match self {
            LeBlancObjectData::Group(group) => Some(group.clone()),
            _ => None
        }
    }

    fn ref_data(&self) -> Option<&LeblancGroup> {
        match self {
            LeBlancObjectData::Group(group) => Some(group),
            _ => None
        }
    }

    fn mut_data(&mut self) -> Option<&mut LeblancGroup> {
        match self {
            LeBlancObjectData::Group(group) => Some(group),
            _ => None
        }
    }
}

impl Display for PromiseCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "PromiseCell(Promise={}, Promised={})", self.promise.read().to_string(), self.echo.data.to_string())
    }
}

impl Display for LeblancGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Group[{}]", self.promises.iter().cloned().map(|p| p.read().to_string()).collect::<Vec<String>>().join(",\n"))
    }
}

pub fn leblanc_object_group(group: LeblancGroup) -> LeBlancObject {
    LeBlancObject::new(
        LeBlancObjectData::Group(group),
        LeBlancType::Group,
        group_methods(),
        Arc::new(Strawberry::new(FxHashMap::default())),
        VariableContext::empty(),
    )
}

pub fn group_methods() -> Arc<FxHashSet<Method>> {
    let mut hash_set = FxHashSet::default();
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_field_method(), _internal_field_));
    hash_set.insert(group_apply_method());
    hash_set.insert(group_pipe_method());
    hash_set.insert(group_pipe_async_method());
    Arc::new(hash_set)
}

pub fn group_apply_method() -> Method {
    let method_store = MethodStore::new("apply".to_string(), vec![
        LeBlancArgument::default(LeBlancType::Function, 0),
        LeBlancArgument::variable(LeBlancType::Flex, 1)
    ]);
    Method::new(
        method_store,
        _internal_group_apply_,
        BTreeSet::new()
    )
}

pub fn group_pipe_method() -> Method {
    let method_store = MethodStore::new("pipe".to_string(), vec![
        LeBlancArgument::variable(LeBlancType::Flex, 0)
    ]);
    Method::new(
        method_store,
        _internal_group_pipe_,
        BTreeSet::new()
    )
}

pub fn group_pipe_async_method() -> Method {
    let method_store = MethodStore::new("pipe_async".to_string(), vec![
        LeBlancArgument::variable(LeBlancType::Flex, 0)
    ]);
    Method::new(
        method_store,
        _internal_group_pipe_async_,
        BTreeSet::new()
    )
}
