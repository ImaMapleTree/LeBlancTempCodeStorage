use core::fmt::Debug;
use std::{fs, io};
use std::fs::File;
use std::io::{Error, Write};
use crate::leblanc::rustblanc::path::ZCPath;

pub trait LBVirtualFile {
    fn path(&self) -> ZCPath;

    fn exists(&self) -> bool;

    fn is_file(&self) -> bool;

    fn is_dir(&self) -> bool;

    fn get_root(&self) -> ZCPath;
}

pub trait LBFileTrait: LBVirtualFile + Debug {
    fn read(&self) -> Result<String, Error>;

    fn read_mut(&mut self) -> Result<String, Error>;

    fn write(&self, buf: &[u8]) -> io::Result<()>;

    fn parent(&self) -> Option<LBFile>;

    fn get_file(&self, name: &str) -> Option<LBFile>;

    fn subdirs(&self) -> Vec<ZCPath>;

    fn subdirs_mut(&mut self) -> Vec<ZCPath>;

    fn subdirs_ref(&mut self) -> Option<&Vec<ZCPath>>;

    fn files(&self) -> Vec<LBFile>;

    fn files_mut(&mut self) -> Vec<LBFile>;
}


/**
Light-weight internal implementation of "File" type in LeBlanc.
**/
#[derive(Clone, Hash, Default, PartialEq, Eq, Debug)]
pub struct LBFile {
    path: ZCPath,
    content: Option<String>
}

impl LBFile {
    pub fn new(path: ZCPath) ->LBFile {
        LBFile { path, content: None }
    }
}

impl LBVirtualFile for LBFile {
    fn path(&self) -> ZCPath {
        self.path
    }

    fn exists(&self) -> bool {
        self.path.as_ref().exists()
    }

    fn is_file(&self) -> bool {
        self.path.as_ref().is_file()
    }

    fn is_dir(&self) -> bool {
        self.path.as_ref().is_dir()
    }

    fn get_root(&self) -> ZCPath {
        match self.parent() {
            None => ZCPath::new("./"),
            Some(p) => p.path()
        }
    }
}


impl LBFileTrait for LBFile {
    fn read(&self) -> Result<String, Error> {
        match &self.content {
            None => Ok(fs::read_to_string(self.path)?),
            Some(content) => Ok(content.clone())
        }
    }

    /**
    <b>For Compilation Use Only</b><br>
    Reads file content to inner buffer variable. After a successful execution data cannot be changed by this function.
    <br>
    <br>
    Refer to read_dynamic if the data should be updated on read.
    **/
    fn read_mut(&mut self) -> Result<String, Error> {
        match &self.content {
            None => {
                let content = fs::read_to_string(self.path)?;
                self.content = Some(content.clone());
                Ok(content)

            }
            Some(content) => {
                Ok(content.clone())
            }
        }
    }

    fn write(&self, buf: &[u8]) -> io::Result<()> {
        File::options().truncate(true).write(true).create(true).open(self.path)?.write_all(buf)
    }

    fn parent(&self) -> Option<LBFile> {
        self.path.parent().map(|path| LBFile::new(ZCPath::new(path.display())))
    }

    fn get_file(&self, name: &str) -> Option<LBFile> {
        if !self.is_dir() { return None; }
        match glob::glob(&self.path.append(name).to_string()) {
            Err(_) => None,
            Ok(mut results) => {
                results.find_map(|r| r.map_or_else(|e| None, Some))
                    .map(|buf| ZCPath::new(buf.display()).as_file())
            }
        }
    }

    fn subdirs(&self) -> Vec<ZCPath> {
        match self.is_dir() {
            false => vec![],
            true => {
                let mut files = Vec::new();
                match glob::glob(&self.path.append("*").to_string()) {
                    Err(_) => {}
                    Ok(results) => {
                        for result in results {
                            match result {
                                Err(_) => {}
                                Ok(path) => files.push(ZCPath::new(path.display())),
                            }
                        }
                    }
                }
                files
            }
        }
    }

    fn subdirs_mut(&mut self) -> Vec<ZCPath> {
        self.subdirs()
    }

    fn subdirs_ref(&mut self) -> Option<&Vec<ZCPath>> {
        None
    }

    fn files(&self) -> Vec<LBFile> {
        match self.is_dir() {
            false => vec![],
            true => {
                let mut files = Vec::new();
                match glob::glob(&self.path.append("*").to_string()) {
                    Err(_) => {}
                    Ok(results) => {
                        for result in results {
                            match result {
                                Err(_) => {}
                                Ok(path) => files.push(ZCPath::new(path.display()).as_file()),
                            }
                        }
                    }
                }
                files
            }
        }
    }

    fn files_mut(&mut self) -> Vec<LBFile> {
        self.files()
    }
}