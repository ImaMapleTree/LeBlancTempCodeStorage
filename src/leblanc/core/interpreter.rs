use std::collections::BTreeSet;


use crate::leblanc::core::bytecode::LeblancBytecode;
use crate::leblanc::core::internal::methods::builtins::create_builtin_function_objects;
use crate::leblanc::core::interpreter::leblanc_runner::LeBlancRunner;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::lib::get_core_modules;
use crate::leblanc::rustblanc::strawberry::print_counts;

pub mod instructions;
pub mod interactive;
pub mod instruction_execution;
pub mod leblanc_runner;


pub fn run(mut bytecode: LeblancBytecode) {
    let mut globals = create_builtin_function_objects();

    let core_modules = get_core_modules();
    for import in bytecode.file_header().imports() {
        if let Some(module) = core_modules.iter().find(|module| module.name == import) {
            globals.append(&mut module.methods_as_objects());
        }
    }

    for mut function in bytecode.body().functions() {
        let arguments = &function.arguments();
        let name = function.name();
        let leblanc_handle = LeblancHandle::from_function_bytecode(function);
        let method_store = MethodStore::new(name.clone(), LeBlancArgument::from_positional(arguments));
        let method = Method::of_leblanc_handle(method_store, leblanc_handle, BTreeSet::new());
        let mut lbo = internal_method(method);
        lbo.context.file = bytecode.file_header().get_file_name();
        if name != "__GLOBAL__" {
            globals.push(lbo.to_mutex());
        }
    }

    let mut runner = LeBlancRunner::new(globals);

    runner.run_main();



}