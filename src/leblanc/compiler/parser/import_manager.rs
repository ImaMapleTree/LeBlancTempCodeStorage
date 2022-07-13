use alloc::vec::IntoIter;
use core::slice::Iter;
use std::collections::HashMap;
use std::iter::FilterMap;
use std::path::PathBuf;
use glob::{glob, Paths};
use crate::compile2;
use crate::leblanc::compiler::compile_import;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component};
use crate::leblanc::compiler::parser::parse_structs::{IdentStore, ScopeSet};

pub struct CompiledImport {
    pub name: String,
    pub components: Vec<Component>,
}

pub fn get_leblanc_file(name: &String, directory: Option<String>) -> Option<PathBuf> {
    println!("Searching for: {}", name);
    let glob1 = glob("*.lb").unwrap();
    let glob2 = glob("*.leblanc").unwrap();
    let mut glob_iter = glob1.chain(glob2);
    let path = glob_iter.find_map(|e| {
        let file = e.unwrap();
        if file.is_file() && file.display().to_string().starts_with(name) { Some(file) }
        else { None }
    });
    println!("Path: {:#?}", path);
    path
}

pub fn import(current_module: &mut CompiledImport, sub_imports: Option<Vec<String>>, import: String) -> Vec<CompiledImport> {
    match sub_imports {
        Some(subs) => {
            let mut compiled = vec![];
            let mut allowed_components = vec![];
            let mut imports = compile_import(import.clone(), get_leblanc_file(&import, None).unwrap());
            imports.iter_mut().find_map(|module| if module.name == import { Some(module.components.clone())} else { None })
                .unwrap().into_iter().for_each(|comp| {
                match &comp.data {
                    Cmpnt::Function { header, .. } => {
                        if let Cmpnt::FunctionHeader { name, ..} = &header.data {
                            if subs.contains(name) { allowed_components.push(comp) }
                        }
                    },
                    Cmpnt::Class { name, .. } => { if subs.contains(name) { allowed_components.push(comp) } },
                    Cmpnt::Trait { name, .. } => { if subs.contains(name) { allowed_components.push(comp) } },
                    Cmpnt::Extension { name, .. } => { if subs.contains(name) { allowed_components.push(comp) } },
                    Cmpnt::Import { module, import } => {
                        compiled.append(&mut import_pass(imports.get_mut(0).unwrap(), vec![(module.clone(), import.clone())]));
                    },
                    Cmpnt::ExtImport { .. } => allowed_components.push(comp),
                    Cmpnt::Enum { name, .. } => { if subs.contains(name) { allowed_components.push(comp) } },
                    _ => {}
                }
            });
            current_module.components.append(&mut allowed_components);
            compiled


        }
        None => { // IMPORTANT: This path is if not importing any sub-things aka using module; where module is a lb file
            compile_import(import.clone(), get_leblanc_file(&import, None).unwrap())
        }
    }
}

pub fn scan_imports(mut current_module: CompiledImport) -> Vec<CompiledImport> {
    let components = current_module.components.iter().filter_map(|c| {
        if let Cmpnt::Import { module, import } = &c.data {
            Some((module.clone(), import.clone()))
        } else {
            None
        }
    }).collect::<Vec<(String, Option<Vec<String>>)>>();
    let mut imports = import_pass(&mut current_module, components);
    imports.insert(0, current_module);
    imports
}

fn import_pass(current_module: &mut CompiledImport, components: Vec<(String, Option<Vec<String>>)>) -> Vec<CompiledImport> {
    let mut new_components = vec![];
    components.into_iter().for_each(|tuple| {
        new_components.append(&mut import(current_module, tuple.1, tuple.0));
    });
    new_components
}