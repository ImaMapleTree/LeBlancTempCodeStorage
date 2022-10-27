



use lalrpop_util::lalrpop_mod;
use crate::leblanc::compiler::file_system::module::CompileModule;
use crate::leblanc::compiler::generator::CodeGenerator;

use crate::leblanc::rustblanc::outcome::Outcome;
use crate::leblanc::rustblanc::outcome::Outcome::{Failure, Success};
use crate::leblanc::rustblanc::path::ZCPath;



pub mod parser;
pub mod bytecode;
pub mod generator;
pub(crate) mod error;
pub(crate) mod file_system;

lalrpop_mod!(pub lalrpop, "/leblanc/compiler/parser/leblanc.rs");

impl CodeGenerator {
    pub fn compile(&mut self, path: ZCPath) -> Outcome<()> {
        self.file_system.cache_file(path);
        self.compile_recursive(path);
        let name_opt = path.file_stem();
        if name_opt.is_none() { return Failure; }
        let name = name_opt.unwrap().to_string_lossy().to_string();

        let parent_opt = path.parent_path();
        if parent_opt.is_none() { return Failure; }
        let parent = parent_opt.unwrap();

        self.finalize(path, parent.join(name + ".lbbc"))
    }

    pub fn compile_recursive(&mut self, name: ZCPath) -> Outcome<&CompileModule> {
        let file = match self.file_system.get_loaded_file(name) {
            Some(_loaded) => return Failure,
            None => self.file_system.add_loaded_file(CompileModule::new(name.as_file()))
        };
        println!("COMPILING: {}", name);


        match file.parse_components() {
            Ok(g) => {
                self.generate(g);
            },
            Err(err) => {
                self.reporter.parse_error(name, err);
            }
        }
        Success(self.file_system.get_loaded_file(name).unwrap())
    }
}