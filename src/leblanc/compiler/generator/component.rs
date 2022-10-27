use crate::leblanc::compiler::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::compiler::bytecode::ToBytecode;
use crate::leblanc::compiler::error::snippet::ErrorSnippet;

use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::compiler::generator::context::CompileInfo;
use crate::leblanc::compiler::generator::converters::expr_to_typed_var;
use crate::leblanc::compiler::generator::generator_types::{FunctionSignature};
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component};



impl CodeGenerator {
    pub fn determine_component(&mut self, component: &Component) -> Result<CompileInfo, ()> {
        let comp = &component.data;
        match comp {
            Cmpnt::Function { header, body, tags: _b } => {
                self.function_number += 1;
                let mut function_bytecode = FunctionBytecode::default();
                if let Cmpnt::FunctionHeader { name, args, returns } = &header.data {
                    let converted_args = expr_to_typed_var(args);
                    for arg in &converted_args {
                        self.add_type(arg.variable.clone(), arg.typing)?;
                        function_bytecode.add_argument(arg.typing);
                    }
                    //println!("Args: {:#?}", self.type_map);
                    function_bytecode.set_name(name.to_string());
                    returns.iter().for_each(|ty| function_bytecode.add_return(*ty))
                }
                let result = self.determine_statement(body);

                self.get_idents_for_function(self.function_number).into_iter().for_each(|(var, rel)| function_bytecode.add_variable(var, rel as u32));
                self.constant_register.iter().for_each(|item| {
                    let ty = item.to_lb_type();
                    function_bytecode.add_constant(item.to_hex(), ty.enum_id() as u16)
                });
                self.constant_register.clear();

                let lines = self.instruct_gen.take_instructions();
                lines.into_iter().for_each(|mut line| function_bytecode.add_instruction_line(line.generate()));
                self.add_function_bytecode(function_bytecode);
                return result;

            }
            Cmpnt::Class { name: _, super_traits: _, items: _ } => {
                /*let funcs = cmpt_to_function(items);
                let properties = cmpt_to_property(items);
                self.class_map.put(GeneratedClass::new(name.to_owned(), super_traits.clone(), properties, funcs));*/
            }
            Cmpnt::Trait { name: _, super_traits: _, items: _ } => {}
            Cmpnt::Extension { name: _, targets: _, items: _ } => {}
            Cmpnt::Property { typing: _, ident: _, value: _ } => {}
            Cmpnt::Import { module: _, import: _ } => {}
            Cmpnt::ExtImport { module: _, extension: _ } => {}
            Cmpnt::Enum { name: _, type_params: _, items: _ } => {}
            Cmpnt::EnumItem { name: _, nested: _ } => {}
            _ => {}
        }
        Ok(CompileInfo::default())
    }

    pub fn determine_dependencies(&mut self, component: &Component) -> Result<(), ()> {
        match &component.data {
            Cmpnt::Function { header, .. } => self.add_function(FunctionSignature::from_header(header)?)?,
            Cmpnt::Class { .. } => {}
            Cmpnt::Trait { .. } => {}
            Cmpnt::Extension { .. } => {}
            Cmpnt::Property { .. } => {}
            Cmpnt::Import { module, import: _ } => {
                let module_string = module.replace('.', "/");
                let file_path = component.location.file;
                let file = self.get_module_import(file_path, &(module_string + ".lb"));
                match file {
                    None => {
                        let (start, end) = component.location.byte_pos;
                        let snippet = ErrorSnippet::new(file_path, "Analysis Error")
                            .add_primary(start, end, format!("Could not find module \"{}\"", module));
                        self.reporter.add_snippet(snippet);
                    },
                    Some(import_file) => {
                        self.compile_recursive(import_file.path());
                    }
                }
            }
            Cmpnt::ExtImport { .. } => {}
            Cmpnt::Enum { .. } => {}
            Cmpnt::EnumItem { .. } => {}
            _ => {}
        }
        Ok(())

    }
}