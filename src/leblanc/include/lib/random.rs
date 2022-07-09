mod _functions_;

use std::collections::BTreeSet;
use crate::leblanc::core::leblanc_argument::{number_argset};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::module::{CoreModule, ModuleMethod};
use crate::leblanc::include::lib::random::_functions_::{_random_no_arg_, _random_one_arg_, _random_two_arg_};
use crate::LeBlancType;


pub fn random_core_module() -> CoreModule {
   CoreModule::new("random".to_string(), vec![
        ModuleMethod::new(random_no_args(), vec![LeBlancType::Double]),
        ModuleMethod::new(random_one_arg(), vec![LeBlancType::Flex]),
        ModuleMethod::new(random_two_arg(), vec![LeBlancType::Flex])
    ])
}

pub fn random_no_args() -> Method {
    Method::new(
        MethodStore::no_args("random".to_string()),
        _random_no_arg_,
        BTreeSet::new()
    )
}

pub fn random_one_arg() -> Method {
    Method::new(
        MethodStore::new(
            "random".to_string(),
            number_argset(0)
        ),
        _random_one_arg_,
        BTreeSet::new()
    )
}

pub fn random_two_arg() -> Method {
    let mut args = number_argset(0);
    args.append(&mut number_argset(1));
    Method::new(
        MethodStore::new(
            "random".to_string(),
            args,
        ),
        _random_two_arg_,
        BTreeSet::new()
    )
}