use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};

use std::sync::{Arc};




use crate::leblanc::core::internal::methods::internal_class::{_internal_to_string_};
use crate::leblanc::core::internal::methods::internal_math::_internal_add_number_;
use crate::leblanc::core::heap::{heap};
use crate::leblanc::core::leblanc_argument::{LeBlancArgument, number_argset};
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::LeBlancType::*;
use crate::leblanc::rustblanc::better_static::BetterStatic;
use crate::leblanc::rustblanc::memory::heap::HeapRef;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

/*static mut BASE_METHODS: BetterStatic<HeapRef<'static, FxHashSet<Method>>> = BetterStatic::new(|| wild_heap().alloc_with(FxHashSet::<Method>::default));

static mut EMPTY_MEMBERS: BetterStatic<HeapRef<'_, FxHashMap<std::string::String, LBObject>>> = BetterStatic::new(|| wild_heap().alloc_with(FxHashMap::default));*/

pub trait ToLeblanc {
    fn create(&self) -> LeBlancObject;
    fn create_mutex(&self) -> LBObject;
}


/*fn base_method_setup() -> HeapRef<'static, FxHashSet<Method>> {
    let mut hash_set = wild_heap().alloc_with(|| FxHashSet::with_capacity_and_hasher(6, FxBuildHasher::default()));
    hash_set.insert(Method::default(base_to_string_method(), _internal_to_string_));
    //hash_set.insert(Method::default(base_expose_method(), _internal_expose_));
    hash_set.insert(Method::default(base_equals_method(), _internal_to_string_));
    hash_set.insert(Method::default(base_clone_method(), _internal_to_string_));
    //hash_set.insert(Method::default(base_field_method(), _internal_field_));
    hash_set.insert( base_addition_method());
    hash_set
}*/

#[inline(always)]
pub fn base_methods() -> HeapRef<'static, FxHashSet<Method>> {
    //return unsafe {BASE_METHODS.access()}.clone()
    HeapRef::default()
}

pub fn internal_method(method: Method) -> LBObject {
    /*let mut methods = wild_heap().alloc_with(|| (*base_methods()).clone());
    methods.insert(method.clone());*/
    /*heap().access().alloc(
    LeBlancObject {
        data: LeBlancObjectData::Function(Box::new(method)),
        typing: Function,
        members: UnsafeVec::default(),
    })*/
    panic!()
}

pub fn base_to_string_method() -> MethodStore {
    MethodStore::no_args("to_string".to_string())
}

pub fn base_expose_method() -> MethodStore {
    MethodStore::no_args("expose".to_string())
}

pub fn base_equals_method() -> MethodStore {
    MethodStore {
        name: "equals".to_string(),
        arguments: vec![LeBlancArgument::default(Flex, 0)],
    }
}

pub fn base_clone_method() -> MethodStore {
    MethodStore::no_args("clone".to_string())
}

pub fn base_field_method() -> MethodStore { MethodStore::new("field".to_string(),
                                                                vec![LeBlancArgument::default(LeBlancType::String, 0)])}


pub fn base_addition_method() -> Method {
    let method_store = MethodStore::new("_ADD_".to_string(), number_argset(0));
    Method::new(
        method_store,
        _internal_add_number_,
        MethodTag::Addition.singleton()
    )
}