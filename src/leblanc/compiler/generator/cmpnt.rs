
use crate::leblanc::compiler::generator::bytecode_generator::BytecodeGenerator;
use crate::leblanc::compiler::generator::converters::{cmpt_to_function, cmpt_to_property, expr_to_typed_var};
use crate::leblanc::compiler::generator::generator_types::{GeneratedClass, GeneratedFuncHeader};
use crate::leblanc::compiler::generator::statement::determine_statement;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component};


impl BytecodeGenerator {
    pub fn determine_component(&mut self, component: &Component) {
        let comp = &component.data;
        match comp {
            Cmpnt::Function { header, body, tags } => {
                if let Cmpnt::FunctionHeader { name, args, returns } = &header.data {
                    let cvrt_args = expr_to_typed_var(args);
                    self.func_map.put(GeneratedFuncHeader::from_typed_variable(name.to_owned(), cvrt_args, returns.clone()));
                }
                self.determine_statement(body);
            }
            Cmpnt::Class { name, super_traits, items } => {
                let funcs = cmpt_to_function(items);
                let properties = cmpt_to_property(items);
                self.class_map.put(GeneratedClass::new(name.to_owned(), super_traits.clone(), properties, funcs));
            }
            Cmpnt::Trait { name, super_traits, items } => {}
            Cmpnt::Extension { name, targets, items } => {}
            Cmpnt::Property { typing, ident, value } => {}
            Cmpnt::Import { module, import } => {}
            Cmpnt::ExtImport { module, extension } => {}
            Cmpnt::Enum { name, type_params, items } => {}
            Cmpnt::EnumItem { name, nested } => {}
            _ => {}
        }
    }
}