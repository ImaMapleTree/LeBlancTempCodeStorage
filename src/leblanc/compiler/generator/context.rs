use std::hash::{Hash, Hasher};
use crate::leblanc::compiler::parser::ast::{Ident};
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Debug)]
pub struct TypeContext {
    pub level: u64,
    pub function: u64,
    pub ident: Ident,
}

impl TypeContext {
    pub fn new(level: u64, function: u64, ident: Ident) -> TypeContext {
        TypeContext { level, function, ident }
    }
}

impl Hash for TypeContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.data.hash(state)
    }
}

impl PartialEq for TypeContext {
    fn eq(&self, other: &Self) -> bool {
        if self.function != other.function { return false; }
        if self.ident.location.file != other.ident.location.file { return false; }
        if self.ident.data != other.ident.data { return false; }
        self.level >= other.level
    }
}

impl Eq for TypeContext {}


#[derive(Default, Debug)]
pub struct CompileInfo {
    types: Vec<LeBlancType>,
    ident: Option<Ident>,
    pub id: usize
}

impl CompileInfo {
    pub fn new(ident: Ident, ty: LeBlancType, id: usize) -> CompileInfo {
        let types = vec![ty];
        CompileInfo { ident: Some(ident), types, id }
    }

    pub fn of(ty: LeBlancType, id: usize) -> CompileInfo {
        let types = vec![ty];
        CompileInfo { ident: None, types, id }
    }

    pub fn of_type(ty: LeBlancType) -> CompileInfo {
        CompileInfo { ident: None, types: vec![ty], id: usize::MAX}
    }

    pub fn ident(ident: Ident, id: usize) -> CompileInfo {
        CompileInfo { types: Vec::new(), ident: Some(ident), id }
    }

    pub fn with_type(mut self, ty: LeBlancType) -> CompileInfo {
        self.types.push(ty);
        self
    }

    pub fn with_types(mut self, types: Vec<LeBlancType>) -> CompileInfo {
        self.types.extend(types);
        self
    }

    pub fn with_ident(mut self, ident: Ident) -> CompileInfo {
        self.ident = Some(ident);
        self
    }

    pub fn get_type(&self) -> LeBlancType {
        match self.types.get(0) {
            None => LeBlancType::Null,
            Some(ty) => *ty
        }
    }

    pub fn get_types(&mut self) -> Vec<LeBlancType> {
        self.types.clone()
    }

    pub fn get_ident(&self) -> Option<&Ident> {
        match &self.ident {
            None => None,
            Some(id) => Some(id)
        }
    }
}

#[derive(Default, Debug)]
pub struct ConstInfo {
    pub id: usize,
    pub ty: LeBlancType,
}

impl ConstInfo {
    pub fn new(id: usize, ty: LeBlancType) -> Self {
        ConstInfo { id, ty }
    }
}

#[derive(Default, Debug)]
pub struct ConditionalInfo {
    pub condition: u8
}