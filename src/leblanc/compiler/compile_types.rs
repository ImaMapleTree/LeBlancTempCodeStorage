use crate::LeBlancType;

pub mod stub_compiler;
pub mod full_compiler;
pub mod full_reader;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CompilationMode {
    Full,
    StubFile,
    ByteCode,
    Realtime
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExtensionExport {
    name: String,
    types: Vec<LeBlancType>
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Import {
    pub name: String,
    pub sub_imports: Vec<String>,
    pub extension: bool
}

impl Import {
    pub fn new(name: String, sub_imports: Vec<String>, extension: bool) -> Import {
        return Import {
            name,
            sub_imports,
            extension
        }
    }
}