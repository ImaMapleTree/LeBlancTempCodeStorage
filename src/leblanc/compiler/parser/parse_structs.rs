use alloc::rc::Rc;
use core::fmt::{Display, Formatter};
use core::slice::Iter;
use std::cell::RefCell;
use std::collections::HashSet;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct SyntaxError {

}

#[derive(Hash, PartialEq, Debug, Eq, Clone)]
pub struct ScopeValue {
    pub scope: ScopeType,
    pub arg_types: Vec<LeBlancType>,
    pub types: Vec<LeBlancType>,
    pub id: usize,
}

#[derive(Clone, PartialEq, Hash, Default, Debug, Eq)]
pub enum ScopeType {
    #[default]
    Global,
    Class,
    Local(u64, Box<ScopeType>),
}

impl ScopeType {
    pub fn get_parent(&self) -> Option<Box<ScopeType>> {
        if let ScopeType::Local(v, p) = self {
            return Some(p.clone());
        }
        return None
    }
}


#[derive(Default, Debug)]
pub struct ScopeSet {
    inner_map: Vec<ScopeValue>
}



impl ScopeSet {
    pub fn new() -> Self {
        ScopeSet {
            inner_map: Vec::new()
        }
    }

    pub fn get_matching_scope_value(&self, s: ScopeType) -> ScopeValue {
        self.inner_map.iter().find(|i| i.scope == s).cloned().unwrap()
    }

    pub fn get_any_scope_value(&self, value: &ScopeType) -> Option<&ScopeValue> {
        match self.inner_map.iter().find(|v| v.scope == *value) {
            None => {}
            Some(val) => return Some(val)
        }
        let mut parent = value.get_parent();
        while parent.is_some() {
            let unwrapped = *parent.unwrap();

            match self.inner_map.iter().find(|v| v.scope == unwrapped) {
                None => {}
                Some(val) => return Some(val)
            }
            parent = unwrapped.get_parent();
        }
        None
    }

    pub fn get_first_id(&self) -> Option<usize> {
        if self.inner_map.is_empty() { return None }
        Some(self.inner_map.iter().next().unwrap().id)
    }

    pub fn set_first_id(&mut self, id: usize)  {
        self.inner_map.iter_mut().next().unwrap().id = id;
    }

    pub fn iter(&self) -> Iter<'_, ScopeValue> {
        self.inner_map.iter()
    }

    pub fn insert(&mut self, value: ScopeValue) {
        self.inner_map.push(value);
    }

    pub fn invalid_in_scope(&mut self, value: &ScopeValue) -> bool {
        if self.inner_map.iter().any(|v| v.scope == value.scope) {
            return true;
        }
        let mut parent = value.scope.get_parent();
        if parent.is_none() {
            return false;
        }
        while parent.is_some() {
            let unwrapped = *parent.unwrap();
            if self.inner_map.iter().any(|v| v.scope == unwrapped) {
                return true;
            }
            parent = unwrapped.get_parent();
        }
        return false;
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ScopeTrack {
    pub scope_occurrence: u64,
    scope: ScopeType
}

impl ScopeTrack {
    pub fn get_scope_type(&self) -> ScopeType {
        self.scope.clone()
    }

    pub fn bump(&mut self) -> ScopeTrack {
        let mut cloned = self.clone();
        cloned.scope = ScopeType::Local(self.scope_occurrence, Box::new(cloned.scope));
        self.scope_occurrence += 1;
        cloned
    }

    pub fn rc(self) -> Rc<RefCell<ScopeTrack>> {
        Rc::new(RefCell::new(self))
    }
}


#[derive(Clone, Hash, PartialEq, PartialOrd, Debug, Eq)]
pub enum IdentStore {
    Function(String, Vec<LeBlancType>, FunctionType),
    Variable(String)
}

#[derive(Clone, Hash, PartialEq, PartialOrd, Debug, Eq, Copy)]
pub enum FunctionType {
    Linked,
    LeBlanc,
}

impl Display for IdentStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IdentStore {
    pub fn get_ident(&self) -> &String {
        match self {
            IdentStore::Function(str, _, _) => str,
            IdentStore::Variable(str) => str
        }
    }
}