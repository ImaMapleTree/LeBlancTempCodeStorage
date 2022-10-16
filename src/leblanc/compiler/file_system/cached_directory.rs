use std::hash::{Hash};
use std::io::Error;
use crate::leblanc::rustblanc::lb_file::{LBFile, LBVirtualFile, LBFileTrait};
use crate::leblanc::rustblanc::path::ZCPath;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct CachedDirectory {
    backing: LBFile,
    cache: Vec<ZCPath>,
}

impl CachedDirectory {
    pub fn new(path: ZCPath) -> CachedDirectory {
        CachedDirectory { backing: LBFile::new(path), cache: Vec::new() }
    }

    pub fn from(file: LBFile) -> CachedDirectory {
        CachedDirectory { backing: file, cache: Vec::new() }
    }
}

impl LBVirtualFile for CachedDirectory {
    fn path(&self) -> ZCPath {
        self.backing.path()
    }

    fn exists(&self) -> bool {
        self.backing.exists()
    }

    fn is_file(&self) -> bool {
        self.backing.is_file()
    }

    fn is_dir(&self) -> bool {
        self.backing.is_dir()
    }

    fn get_root(&self) -> ZCPath {
        self.backing.get_root()
    }
}

impl LBFileTrait for CachedDirectory {
    fn read(&self) -> Result<String, Error> {
        self.backing.read()
    }

    fn read_mut(&mut self) -> Result<String, Error> {
        self.backing.read_mut()
    }

    fn write(&self, buf: &[u8]) -> std::io::Result<()> {
        self.backing.write(buf)
    }

    fn parent(&self) -> Option<LBFile> {
        self.backing.parent()
    }

    fn get_file(&self, name: &str) -> Option<LBFile> {
        self.backing.get_file(name)
    }

    fn subdirs(&self) -> Vec<ZCPath> {
        self.backing.subdirs()
    }

    fn subdirs_mut(&mut self) -> Vec<ZCPath> {
        if self.cache.is_empty() {
            self.cache = self.backing.subdirs();
        }
        self.cache.clone()
    }

    fn subdirs_ref(&mut self) -> Option<&Vec<ZCPath>> {
        if self.cache.is_empty() {
            self.cache = self.backing.subdirs();
        }
        Some(&self.cache)
    }

    fn files(&self) -> Vec<LBFile> {
        self.backing.files()
    }

    fn files_mut(&mut self) -> Vec<LBFile> {
        self.backing.files_mut()
    }
}

