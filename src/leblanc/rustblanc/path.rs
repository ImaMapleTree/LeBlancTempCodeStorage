use core::fmt::{Debug, Display, Formatter};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use crate::leblanc::rustblanc::copystring::{CopyString, CopyStringable};
use crate::leblanc::rustblanc::lb_file::LBFile;
use serde::{Serialize};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Default, Serialize)]
pub struct ZCPath {
    path: CopyString,
}

impl ZCPath {
    pub const fn constant() -> ZCPath {
        ZCPath {
            path: CopyString::constant()
        }
    }

    pub fn new<T: Display>(path: T) -> ZCPath {
        ZCPath {
            path: CopyString::from(path.to_string().replace("\\", "/"))
        }
    }

    pub fn append<T: AsRef<str>>(&self, ending: T) -> ZCPath {
        ZCPath::from(self.path.to_string() + "/" + ending.as_ref())
    }

    pub fn join<T: Display>(&self, path: T) -> ZCPath {
        ZCPath::new(self.as_ref().join(path.to_string()).display())
    }

    pub fn copy_string(&self) -> CopyString { self.path }

    pub fn parent(&self) -> Option<&Path> {
        Path::new(&self.path).parent()
    }

    pub fn parent_path(&self) -> Option<ZCPath> {
        Some(ZCPath::from(self.as_ref().parent()?.to_string_lossy().as_ref()))
    }

    pub fn as_file(&self) -> LBFile {
        LBFile::new(*self)
    }

    pub fn file_prefix(&self) -> Option<&OsStr> {
        self.as_ref().file_prefix()
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.as_ref().file_name()
    }

    pub fn file_stem(&self) -> Option<&OsStr> {
        self.as_ref().file_stem()
    }

    pub fn extension(&self) -> Option<&OsStr> {
        self.as_ref().extension()
    }

}

impl From<String> for ZCPath {
    fn from(path: String) -> Self {
        ZCPath {
            path: CopyString::from(path.replace("\\", "/"))
        }
    }
}

impl From<&String> for ZCPath {
    fn from(path: &String) -> Self {
        ZCPath {
            path: CopyString::from(path.replace("\\", "/"))
        }
    }
}

impl From<&str> for ZCPath {
    fn from(path: &str) -> Self {
        ZCPath {
            path: CopyString::from(path.replace("\\", "/"))
        }
    }
}

impl From<PathBuf> for ZCPath {
    fn from(path: PathBuf) -> Self {
        ZCPath::from(path.to_string_lossy().as_ref())
    }
}

impl AsRef<Path> for ZCPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl Display for ZCPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Debug for ZCPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ZCPath(\"{}\")", self.path)
    }
}