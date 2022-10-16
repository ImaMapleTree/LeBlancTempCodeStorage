use std::collections::BTreeSet;
use std::path::PathBuf;


use crate::leblanc::compiler::bytecode::LeblancBytecode;
use crate::leblanc::compiler::parser::import_manager;
use crate::leblanc::core::internal::methods::builtins::create_builtin_function_objects;
use crate::leblanc::core::interpreter::leblanc_runner::LeBlancRunner;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::copystring::CopyString;
use crate::leblanc::include::lib::get_core_modules;

pub mod instructions;
pub mod interactive;
pub mod instruction_execution;
pub mod leblanc_runner;
pub mod instructions2;
pub mod instruction_execution2;


pub fn run(mut bytecode: LeblancBytecode) {
    let mut globals = create_builtin_function_objects();

    for mut function in bytecode.body().functions() {
        let arguments = &function.arguments();
        let name = function.name();
        let leblanc_handle = LeblancHandle::from_function_bytecode(function);
        let method_store = MethodStore::new(name.clone(), LeBlancArgument::from_positional(arguments));
        let method = Method::of_leblanc_handle(method_store, leblanc_handle, BTreeSet::new());
        let mut lbo = internal_method(method);
        lbo.context.file = CopyString::new(bytecode.file_header().get_file_name());
        if name != "__GLOBAL__" {
            globals.push(lbo.to_mutex());
        }
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