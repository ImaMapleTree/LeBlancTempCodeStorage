pub(crate) mod generator_types;
pub(crate) mod dependency;
mod converters;
mod component;
mod statement;
mod expression;
mod context;
mod conditional;
mod constant;
mod instruction_generator;


use core::fmt::{Debug, Formatter};
use std::collections::{HashMap};
use std::fs::File;
use std::io::Write;

use std::mem::take;
use crate::bytes;
use crate::leblanc::compiler::bytecode::file_body::FileBodyBytecode;
use crate::leblanc::compiler::bytecode::file_header::FileHeaderBytecode;
use crate::leblanc::compiler::bytecode::function_bytes::FunctionBytecode;

use crate::leblanc::compiler::bytecode::{LeblancBytecode, ToBytecode};
use crate::leblanc::compiler::error::{ErrorReporter};
use crate::leblanc::compiler::error::snippet::ErrorSnippet;
use crate::leblanc::compiler::file_system::LBFileSystem;
use crate::leblanc::compiler::generator::context::{CompileInfo, TypeContext};

use crate::leblanc::compiler::generator::generator_types::{GeneratedClass, FunctionSignature};
use crate::leblanc::compiler::generator::instruction_generator::InstructionGenerator;
use crate::leblanc::compiler::parser::ast::{Component, Const, Ident, Location};
use crate::leblanc::core::internal::methods::builtins::create_lazy_functions;
use crate::leblanc::core::interpreter::instructions2::Instruction2::LOAD_FUNCTION;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::native_types::LeBlancType;

use crate::leblanc::rustblanc::component_map::ComponentMap;
use crate::leblanc::rustblanc::lazy_store::{LazyStore, Strategy};
use crate::leblanc::rustblanc::lb_file::{LBFileTrait};
use crate::leblanc::rustblanc::outcome::Outcome;
use crate::leblanc::rustblanc::outcome::Outcome::{Failure, Success};
use crate::leblanc::rustblanc::path::ZCPath;

/*
TODO:
    logic is pretty okay but there's a couple of things that need to be worked out before skeleton system operates
    a) fork LBFile to Directory so that we can cache files
    b) skeleton looks in relevant directories adding to the "directories" hashset and loading more skeletons
 */

pub struct CodeGenerator {
    header: FileHeaderBytecode,
    body: FileBodyBytecode,
    pub functions: Vec<FunctionBytecode>,
    pub instruct_gen: InstructionGenerator,
    pub file_system: LBFileSystem,
    class_map: ComponentMap<GeneratedClass>,
    pub func_map: LazyStore<FunctionSignature>,
    pub type_map: HashMap<TypeContext, (LeBlancType, usize)>,
    constant_register: LazyStore<Const>,
    pub level: u64,
    pub function_number: u64,
    pub reporter: ErrorReporter,
}

impl CodeGenerator {
    pub fn generate(&mut self, components: Vec<Component>) {
        let component_file = File::options().truncate(true).write(true).create(true).open("components.json");
        let _s: String = serde_json::to_string(&components).unwrap();
        component_file.unwrap().write_all(_s.as_bytes()).unwrap();
        for component in &components {
            self.determine_dependencies(component);
        }
        for component in &components {
            self.determine_component(component);
        }
    }

    pub fn add_function_bytecode(&mut self, bytecode: FunctionBytecode) {
        self.functions.push(bytecode);
    }

    pub fn finalize(&mut self, input: ZCPath, output: ZCPath) -> Outcome<()> {
        if self.reporter.has_errors() {
            self.reporter.report();
            return Failure;
        }
        let mut header = FileHeaderBytecode::default();
        header.set_file_name(&input.to_string());

        let mut body = FileBodyBytecode::default();
        take(&mut self.functions).into_iter().for_each(|func| body.add_function(func));

        let mut bytecode = LeblancBytecode::new(header, body);
        let output = output.as_file().write(&hex::decode(bytecode.generate().to_string()).unwrap());
        if output.is_err() { return Failure; }

        Success(())
    }


    pub fn add_function(&mut self, func: FunctionSignature) -> Result<(), ()> {
        let (start, end) = func.byte_pos();
        match self.func_map.similar(&func, Strategy::STANDARD) {
            None => {
                self.func_map.add(func);
                Ok(())
            }
            Some(existing) => {
                let (start2, end2) = existing.byte_pos();
                let snippet = ErrorSnippet::new(func.file(), "Analysis Error")
                    .add_primary(start, end,"Function has identical signature to existing function")
                    .add_secondary(start2, end2, "First occurrence of signature here");
                self.reporter.add_snippet(snippet);
                Err(())
            }
        }
    }

    pub fn get_module_import(&self, path: ZCPath, module: &String) -> Option<&Box<dyn LBFileTrait>> {
        self.file_system.get_import_file(path, module)
    }

    pub fn get_idents_for_function(&self, number: u64) -> Vec<(String, usize)> {
        self.type_map.iter().filter_map(|(key, (_ty, id))| {
            if key.function == number {
                Some((key.ident.resolve(), *id))
            } else {
                None
            }
        }).collect()
    }

    //noinspection ALL
    //noinspection RsExternalLinter
    pub fn add_type(&mut self, id: Ident, ty: LeBlancType) -> Result<CompileInfo, ()> {
        let file_path = id.location.file;
        let context = TypeContext::new(self.level, self.function_number,id.clone());
        //noinspection RsExternalLinter
        if let Some((old_context, existing_data)) = self.type_map.get_key_value(&context) {
            let (old_type, _var_id) = existing_data;
            let mut snippet = ErrorSnippet::new(file_path, "Analysis Error");
            let (start, end) = id.location.byte_pos;
            let (start2, end2) = old_context.ident.location.byte_pos;
            snippet = snippet.add_primary(start - 1 - ty.to_string().len(), end, "Cannot redeclare variable")
                .add_secondary(start2 - 1 - old_type.to_string().len(), end2, "Previously declared here");
            self.reporter.add_snippet(snippet);
            Err(())
        } else {
            let var_id = self.get_idents_for_function(self.function_number).len();
            self.type_map.insert(context, (ty, var_id));
            Ok(CompileInfo::new(id, ty, var_id))
        }
    }

    pub fn get_type(&self, id: &Ident) -> Result<CompileInfo, ()> {
        let mut context = TypeContext::new(self.level, self.function_number,id.clone());
        let existing = self.type_map.get(&context);
        if existing.is_none() {
            if let Some(other_func) = self.func_map.index(&FunctionSignature::new(&id.resolve(), vec![], vec![], Default::default()), Strategy::LAZY) {
                return Ok(CompileInfo::new(take(&mut context.ident), LeBlancType::Function, other_func))
            }
            return Err(()); }
        let (ty, id) = existing.unwrap();
        Ok(CompileInfo::new(take(&mut context.ident), *ty, *id))
    }

    pub fn validate_type(&mut self, id: Ident, location: Location, ty: LeBlancType, err_on_undef: bool) -> Result<CompileInfo, ()> {
        let (start, end) = id.location.byte_pos;
        let (start2, end2) = location.byte_pos;
        let file_path = id.location.file;
        let context = TypeContext::new(self.level, self.function_number,id);
        match self.type_map.get(&context) {
            None => {
                if err_on_undef {
                    let snippet = ErrorSnippet::new(file_path, "Analysis Error")
                        .add_primary(start2, end2,"Cannot assign value to undefined variable");
                    self.reporter.add_snippet(snippet);
                    Err(())
                } else { Ok(CompileInfo::of_type(ty)) }
            }
            Some(var_info) => {
                let (lb_type, var_id) = var_info;
                if *lb_type != ty {
                    let snippet = ErrorSnippet::new(file_path, "Analysis Error")
                        .add_primary(start2, end2, "Incompatible Variable Types")
                        .add_primary(start2, end2, format!("- type \"{}\"", ty))
                        .add_secondary(start, end, format!("Previously declared as type \"{}\"", lb_type));
                    self.reporter.add_snippet(snippet);
                    Err(())
                } else { Ok(CompileInfo::of(ty, *var_id)) }
            }
        }
    }
}



impl Debug for CodeGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BytecodeGenerator")
            .field("class_map", &self.class_map)
            .field("func_map", &self.func_map)
            .field("type_map", &self.type_map)
            .finish()
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        CodeGenerator {
            header: Default::default(),
            body: Default::default(),
            functions: Default::default(),
            instruct_gen: Default::default(),
            file_system: Default::default(),
            class_map: Default::default(),
            func_map: create_lazy_functions(),
            type_map: Default::default(),
            constant_register: Default::default(),
            level: Default::default(),
            function_number: Default::default(),
            reporter: Default::default()
        }
    }
}