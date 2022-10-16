use std::hash::{Hash, Hasher};
use crate::leblanc::rustblanc::path::ZCPath;
use crate::leblanc::rustblanc::rust_override::OptionEquality;

#[derive(Default, Copy, Clone, Debug)]
pub struct DependencyPath {
    real_path: ZCPath,
    export_path: Option<ZCPath>
}

impl DependencyPath {
    pub fn new(real_path: ZCPath, export_path: Option<ZCPath>) -> DependencyPath {
        DependencyPath { real_path, export_path}
    }

    pub fn real(mut self, path: ZCPath) -> Self {
        self.real_path = path; self
    }

    pub fn export(mut self, path: ZCPath) -> Self {
        self.export_path = Some(path); self
    }

    pub fn get_real(&self) -> ZCPath {
        self.real_path
    }

    pub fn get_export(&self) -> Option<ZCPath> {
        self.export_path
    }

    pub fn has_export(&self) -> bool {
        self.export_path.is_some()
    }
}

impl From<ZCPath> for DependencyPath {
    fn from(path: ZCPath) -> Self {
        DependencyPath { real_path: path, export_path: None }
    }
}

impl From<String> for DependencyPath {
    fn from(path: String) -> Self {
        DependencyPath::from(ZCPath::from(path))
    }
}

impl From<&String> for DependencyPath {
    fn from(path: &String) -> Self {
        DependencyPath::from(ZCPath::from(path))
    }
}

impl Hash for DependencyPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.real_path.hash(state)
    }
}

impl PartialEq for DependencyPath {
    fn eq(&self, other: &Self) -> bool {
        other.export_path.ieq(&Some(self.real_path)) || self.export_path.ieq(&Some(other.real_path)) || self.real_path == other.real_path
    }
}