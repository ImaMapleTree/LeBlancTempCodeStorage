#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Import {
    pub name: String,
    pub source: String,
    pub import_type: ImportType,
}

impl Import {
    pub fn new(name: &String, source: &String, import_type: ImportType) -> Import {
        Import {
            name: name.clone(),
            source: source.clone(),
            import_type
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum ImportType {
    Extension,
    File,
    SubImport
}