use std::collections::BTreeSet;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::module::{CoreModule, ModuleMethod};
use crate::leblanc::rustblanc::lib::datelib::_functions_::{_epoch_, _epoch_seconds_};
use crate::LeBlancType;

mod _functions_;

pub fn datelib_core_module() -> CoreModule {
    CoreModule::new("datelib".to_string(), vec![
        ModuleMethod::new(epoch(), vec![LeBlancType::Double]),
        ModuleMethod::new(epoch_seconds(), vec![LeBlancType::Int64]),
    ])
}

pub fn epoch() -> Method {
    Method::new(
        MethodStore::no_args("epoch".to_string()),
        _epoch_,
        BTreeSet::new()
    )
}

pub fn epoch_seconds() -> Method {
    Method::new(
        MethodStore::no_args("epoch_seconds".to_string()),
        _epoch_seconds_,
        BTreeSet::new()
    )
}