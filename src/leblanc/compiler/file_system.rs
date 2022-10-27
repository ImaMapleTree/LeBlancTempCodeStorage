pub mod cached_directory;
pub mod dependency_path;
pub mod module;

use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader, Read};
use crate::leblanc::compiler::bytecode::LeblancBytecode;


use crate::leblanc::compiler::error::ErrorReporter;
use crate::leblanc::compiler::file_system::cached_directory::CachedDirectory;

use crate::leblanc::compiler::file_system::module::CompileModule;
use crate::leblanc::compiler::generator::dependency::Dependency;
use crate::leblanc::compiler::parser::ast::{Reqs};
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::lb_file::{LBFile, LBFileTrait, LBVirtualFile};
use crate::leblanc::rustblanc::path::ZCPath;
use crate::leblanc::rustblanc::utils::encode_hex;


#[derive(Debug, Default)]
pub struct LBFileSystem {
    dependencies: HashMap<ZCPath, Vec<Dependency>>,
    pub reporter: ErrorReporter,
    cached_files: HashMap<ZCPath, Box<dyn LBFileTrait>>,
    loaded_files: HashMap<ZCPath, CompileModule>
}

impl LBFileSystem {

    pub fn cache_file(&mut self, path: ZCPath) {
        if self.is_loaded_path(path) { return }
        let file = path.as_file();
        if file.is_dir() {
            let cached_file = CachedDirectory::from(file);
            cached_file.subdirs().iter().for_each(|p| self.cache_file(*p));
            self.add_file_to_cache(Box::new(cached_file), true);

        } else {
            if let Some(prefix) = path.file_prefix() {
                if prefix == ".skeleton" {
                    self.load_skeleton(file.clone());
                }
            }
            self.add_file_to_cache(Box::new(file), true);
        }

        if let Some(parent) = path.parent() {
            let parent_path = ZCPath::new(parent.display());
            if !self.is_loaded_path(parent_path) {
                CachedDirectory::new(parent_path).subdirs_ref().unwrap().iter().for_each(|p| self.cache_file(*p));
            }
        }

    }

    pub fn load_skeleton(&mut self, file: LBFile) {
        let path = file.path();
        let parent_path = ZCPath::new(path.parent().unwrap().display());
        let filename = if let Some(prefix) = parent_path.file_prefix() { String::from(prefix.to_string_lossy()) }
        else { String::from("") };
        let loaded = CompileModule::new(file);
        match loaded.parse_required() {
            Err(err) => self.reporter.parse_error(path, err),
            Ok(requirements) => {
                for requirement in requirements {
                    match requirement.data {
                        Reqs::Export { .. } => {}
                        Reqs::Requires { dependees, required } => {
                            for dependee in dependees {
                                let dependee_path = parent_path.join(dependee.resolve());
                                let resolved = dependee.resolve().replacen(&filename, ".", 1);
                                let required_paths = required
                                    .iter()
                                    .filter_map(|i| {
                                        let ri = i.resolve();
                                        if ri != resolved { Some(ri) } else { None }
                                    })
                                    .filter_map(|i|{
                                        let required_path = parent_path.join(&i);
                                        if required_path.as_ref().exists() {
                                            Some(Dependency::new(i, required_path))
                                        } else {
                                            self.get_dependency(parent_path, i).cloned()
                                        }
                                    })
                                    .collect();
                                match self.dependencies.get_mut(&dependee_path) {
                                    None => {
                                        self.dependencies.insert(dependee_path, required_paths);
                                    }
                                    Some(existing_reqs) => {
                                        existing_reqs.extend(required_paths)
                                    }
                                }
                            }
                        }
                    }

                }
            }
        }
    }

    pub fn load_file(_path: ZCPath) -> CompileModule {
        CompileModule::default()
    }

    pub fn is_loaded_path(&self, path: ZCPath) -> bool {
        self.cached_files.values().any(|f| f.path() == path)
    }

    pub fn is_loaded(&self, file: &Box<dyn LBFileTrait>) -> bool {
        self.is_loaded_path(file.path())
    }

    pub fn get_cached(&self, path: ZCPath) -> Option<&Box<dyn LBFileTrait>> {
        self.cached_files.get(&path)
    }

    pub fn get_cached_mut(&mut self, path: ZCPath) -> Option<&mut Box<dyn LBFileTrait>> {
        self.cached_files.get_mut(&path)
    }

    fn add_file_to_cache(&mut self, file: Box<dyn LBFileTrait>, bypass: bool) {
        if bypass || !self.is_loaded(&file) {
            self.cached_files.insert(file.path(), file);
        }
    }

    fn get_dependency(&self, base_path: ZCPath, req: String) -> Option<&Dependency> {
        self.dependencies.get(&base_path)?.iter().find(|dep| dep.matches_req(&req))
    }

    //noinspection ALL
    //noinspection RsExternalLinter
    pub fn get_import_file(&self, base_path: ZCPath, name: &String) -> Option<&Box<dyn LBFileTrait>> {
        let root = ZCPath::new(&base_path.parent()?.to_string_lossy());
        let same_dir_path = root.join(name);
        if same_dir_path.as_ref().exists() {
            return self.cached_files.get(&same_dir_path);
        }
        let dep = self.dependencies.get(&root)?.iter().find_map(|dep| {
            let dep_file_path = dep.get_path().get_real().join(name);
            if dep_file_path.as_ref().exists() { Some(dep_file_path) } else { None }
        })?;
        self.cached_files.get(&dep)
    }

    pub fn add_loaded_file(&mut self, file: CompileModule) -> &mut CompileModule {
        let path = file.path();
        self.loaded_files.insert(path, file);
        self.loaded_files.get_mut(&path).unwrap()
    }

    pub fn get_loaded_file(&self, path: ZCPath) -> Option<&CompileModule> {
        self.loaded_files.get(&path)
    }

    pub fn get_loaded_file_mut(&mut self, path: ZCPath) -> Option<&mut CompileModule> {
        self.loaded_files.get_mut(&path)
    }

    pub fn loaded_file_exists(&self, path: ZCPath) -> bool {
        self.loaded_files.contains_key(&path)
    }
}

pub fn read_file(path: String) -> LeblancBytecode {
    let file = File::open(path.replace(".lb", ".lbbc")).unwrap();
    let file_reader = BufReader::new(file);
    let hex = encode_hex(&file_reader.bytes().map(|l| l.unwrap()).collect::<Vec<u8>>());



    LeblancBytecode::from(hex)
}

pub fn read_bytecode(hex: Hexadecimal) -> LeblancBytecode {
    LeblancBytecode::from(hex)
}