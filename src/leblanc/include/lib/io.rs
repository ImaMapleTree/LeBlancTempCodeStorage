use std::collections::BTreeSet;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::module::{CoreModule, ModuleMethod};
use crate::leblanc::include::lib::io::_functions_::{_stdin_prompt_, _stdin_read_, _stdin_read_int_};
use crate::LeBlancType;

mod _functions_;

pub fn io_core_module() -> CoreModule {
    CoreModule::new("io".to_string(), vec![
        ModuleMethod::new(io_stdin_read(), vec![LeBlancType::String]),
        ModuleMethod::new(io_stdin_prompt(), vec![LeBlancType::String]),
        ModuleMethod::new(io_stdin_read_int(), vec![LeBlancType::Int]),
    ])
}

pub fn io_stdin_read() -> Method {
    Method::new(
        MethodStore::no_args("read".to_string()),
        _stdin_read_,
        BTreeSet::new()
    )
}

pub fn io_stdin_read_int() -> Method {
    Method::new(
        MethodStore::no_args("read_int".to_string()),
        _stdin_read_int_,
        BTreeSet::new()
    )
}

pub fn io_stdin_prompt() -> Method {
    Method::new(
        MethodStore::new("prompt".to_string(),
        vec![LeBlancArgument::default(LeBlancType::String, 0)]
        ),
        _stdin_prompt_,
        BTreeSet::new()
    )
}