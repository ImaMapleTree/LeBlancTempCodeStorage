use crate::leblanc::compiler::file_system::dependency_path::DependencyPath;
use crate::leblanc::rustblanc::path::ZCPath;
use crate::leblanc::rustblanc::rust_override::OptionEquality;

#[derive(Clone, Debug)]
pub struct Dependency {
    name: String,
    path: DependencyPath,
    satisfied: bool
}

impl Dependency {
    pub fn new(name: String, path: ZCPath) -> Self {
        Dependency { name, path: DependencyPath::from(path), satisfied: false }
    }

    pub fn export(name: String, real: ZCPath, export: ZCPath) -> Self {
        Dependency { name, path: DependencyPath::new(real, Some(export)), satisfied: false}
    }

    pub fn is_satisfied(&self) -> bool {
        self.satisfied
    }

    pub fn set_satisfied(&mut self, satisfied: bool) {
        self.satisfied = satisfied
    }

    pub fn matches_req(&self, name: &String) -> bool {
        self.name.eq(name)
    }

    pub fn matches_name(&self, name: &String) -> bool {
        self.path.get_real().file_name().ieq_not_none(&Some(name.as_ref()))
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &DependencyPath {
        &self.path
    }
}