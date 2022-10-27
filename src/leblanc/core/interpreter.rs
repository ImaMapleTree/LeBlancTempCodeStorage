use std::collections::BTreeSet;

use lazy_static::lazy_static;


use crate::leblanc::compiler::bytecode::LeblancBytecode;
use crate::leblanc::configuration::HDEF_MB;

use crate::leblanc::core::internal::methods::builtins::{create_builtin_function_methods, create_builtin_function_objects};
use crate::leblanc::core::interpreter::leblanc_runner::{get_handles, LeBlancRunner};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::copystring::CopyString;
use crate::leblanc::rustblanc::memory::heap::TypedHeap;


pub(crate) mod instructions;
pub(crate) mod interactive;
pub(crate) mod instruction_execution;
pub(crate) mod leblanc_runner;
pub(crate) mod instruction_execution2;
pub(crate) mod execution_context;
pub mod instructions2;
use crate::leblanc::rustblanc::strawberry::Strawberry;

pub(crate) fn run(mut bytecode: LeblancBytecode) {
    get_handles().clear();
    get_handles().push(LeblancHandle::null());
    let mut globals = create_builtin_function_methods();

    for mut function in bytecode.body().functions() {
        let arguments = &function.arguments();
        let name = function.name();
        let index = get_handles().len();
        let leblanc_handle = LeblancHandle::from_function_bytecode(function, index);
        get_handles().push(leblanc_handle);
        //let method_store = MethodStore::new(name.clone(), LeBlancArgument::from_positional(arguments));
        //let method = Method::of_leblanc_handle(method_store, index, BTreeSet::new());
        /*let mut lbo = internal_method(method);
        if name != "__GLOBAL__" {
            globals.push(lbo);
        }*/
    }

    /*for import in bytecode.file_header().imports() {
        let file = import_manager::get_leblanc_file(&import, None);
        match file {
            None => {}
            Some(path) => {
                let module= import_dynamic(path);
                globals.append(&mut module.methods_as_objects());
                Box::leak(Box::new(module));
            },
        }
        /*if let Some(module) = core_modules.iter().find(|module| module.name == import) {
            globals.append(&mut module.methods_as_objects());
        }*/
    }*/

    let mut runner = LeBlancRunner::new(globals);

    runner.run_main();



}