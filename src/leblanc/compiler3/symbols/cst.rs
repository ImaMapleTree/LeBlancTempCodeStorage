use std::ops::Deref;
use std::sync::{Arc};
use std::sync::atomic::AtomicU64;
use parking_lot::Mutex;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::leblanc::compiler3::leblanc_type::LBType;
use crate::leblanc::compiler3::symbols::symbol_table::{LinkedSymbolTable, Symbol, SymbolTable, TableIntention};
use crate::leblanc::compiler3::symbols::symbol_table::Scope::Global;
use crate::leblanc::compiler::parser::ast::Location;
use crate::leblanc::rustblanc::copystring::CopyString;
use crate::leblanc::rustblanc::lazy_store::{Lazy, LazyStore, Strategy};

pub trait SubTable {
    fn create_sub_table(&mut self) -> Self;
    fn get_highest_table(&self) -> Self;
}

pub type LCST = Arc<Mutex<CombinedSymbolTable>>;
pub type SharedContext = Arc<Mutex<LazyStore<CombinedSymbol>>>;

#[derive(Clone, Default, Debug)]
pub struct CombinedSymbolTable {
    pub(crate) symbols: Vec<CombinedSymbol>,
    pub(crate) subtables: Vec<LCST>,
    pub(crate) clashes: LazyStore<CombinedSymbol>,
    pub(crate) shared: Option<SharedContext>,
    pub(crate) parent: Option<LCST>,
    pub(crate) func_counter: Arc<AtomicU64>,
    pub(crate) class_counter: Arc<AtomicU64>,
    pub(crate) level: u16

}

impl Serialize for CombinedSymbolTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let ss: Vec<&mut CombinedSymbolTable> = self.subtables.iter().map(|t| t.data_ptr()).map(|ptr| unsafe {&mut *ptr}).collect();

        let mut state = serializer.serialize_struct("CombinedSymbolTable", 4)?;
        state.serialize_field("symbols", &self.symbols)?;
        state.serialize_field("subtables", &ss)?;
        state.serialize_field("clashes", &self.clashes.iter().collect::<Vec<&CombinedSymbol>>())?;
        state.end()
    }
}

#[derive(Clone, Eq, Serialize, Default, Debug)]
pub struct CombinedSymbol {
    #[serde(flatten)]
    pub(crate) name: CopyString,
    pub(crate) stype: SymbolType,
    pub(crate) typ: Option<LBType>,
    pub(crate) subsymbols: Vec<CombinedSymbol>,
    #[serde(skip_serializing)]
    pub(crate) location: Option<Location>,
    pub(crate) unique: bool,
    pub(crate) scope: Scope,
    #[serde(skip_serializing)]
    pub(crate) linked: Option<LCST>
}

impl PartialEq for CombinedSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.unique == other.unique && self.stype == other.stype
        && self.subsymbols == other.subsymbols && self.location == other.location
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Default, Debug)]
pub enum SymbolType {
    Function(u64),
    Class(u64),
    #[default]
    Standard
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Default, Debug)]
pub enum Scope {
    #[default]
    Global,
    Intermediate,
    Local(u16),
}

impl Lazy for CombinedSymbol {
    /// Compares name, type, and self.scope < other.scope
    fn lazy() -> Strategy {
        Strategy::LAZY
    }

    /// Compares name, type, scope < other.scope, and sub-symbols
    fn standard() -> Strategy {
        Strategy::STANDARD
    }

    /// Compares everything but encapsulation
    fn rust() -> Strategy {
        Strategy::RUST
    }

    /// Always checks unique flag
    fn scan(&self, other: &Self, strategy: Strategy) -> bool {
        if !(self.unique || other.unique) { false }
        else {
            match strategy {
                Strategy::LAZY => {
                    println!("My name: {}, Other name: {}, My type: {:?}, Other type: {:?}, the truth: {}", self.name, other.name, self.typ, other.typ, self.name == other.name && self.typ == other.typ);
                    self.name == other.name && self.typ == other.typ
                }
                Strategy::STANDARD => {
                    //println!("My name: {}, Other name: {}, My type: {:?}, Other type: {:?}, My scope: {:?}, Other Scope: {:?}, My ss: {:?}, Other ss: {:?}, the truth: {}", self.name, other.name, self.typ, other.typ, self.scope, other.scope, self.subsymbols, other.subsymbols, self.name == other.name && self.typ == other.typ && self.scope <= other.scope
                    //    && self.subsymbols == other.subsymbols);
                    self.name == other.name && self.typ == other.typ && self.scope <= other.scope
                    && self.subsymbols == other.subsymbols
                }
                Strategy::RUST => {
                    self.name == other.name && self.typ == other.typ && self.scope <= other.scope
                    && self.subsymbols == other.subsymbols && self.location == other.location
                }
            }
        }
    }
}

impl CombinedSymbolTable {
    pub fn add_symbol(&mut self, mut symbol: CombinedSymbol) {
        if self.level == 0 {
            symbol.scope = Scope::Global;
        }
        self.symbols.push(symbol.clone());
        if symbol.unique {
            if self.check_clashing(&symbol) {
                eprintln!("Clash foud for: {:?}", symbol);
            }
            self.clashes.add(symbol.clone());
            if let Some(shared) = &self.shared {
                shared.lock().get_or_add(symbol, CombinedSymbol::lazy());
            }
        }
    }

    pub fn check_existence(&self, symbol: &CombinedSymbol) -> bool {
        if self.symbols.contains(symbol) { return true; }
        if let Some(parent) = &self.parent {
            return parent.lock().check_existence(symbol);
        }
        false
    }

    pub fn check_clashing(&self, symbol: &CombinedSymbol) -> bool {
        if self.clashes.contains(symbol, Strategy::STANDARD) { return true; }
        if let Some(parent) = &self.parent {
            return parent.lock().check_clashing(symbol);
        }
        false
    }

    pub fn add_to_intermediate(&mut self, symbol: CombinedSymbol) {
        if self.parent.is_none() {

            let index = self.clashes.index(&symbol, Strategy::LAZY);
            if index.is_none() {
                self.clashes.add(symbol);
            }
        } else {
            let parent = self.parent.as_mut();
            parent.unwrap().lock().add_to_intermediate(symbol);
        }
    }

    pub fn create_shared_context(&mut self) {
        self.shared = Some(SharedContext::default());
    }

    fn get_highest_table(&self) -> Option<LCST> {
        if let Some(parent) = &self.parent {
            return Some(parent.get_highest_table());
        }
        None
    }
}

impl SubTable for LCST {
    fn create_sub_table(&mut self) -> Self {
        let mut lock = self.lock();
        let parent = Some(self.clone());
        let level = lock.level + 1;
        let sub_table = Arc::new(Mutex::new(CombinedSymbolTable {
            symbols: vec![],
            subtables: vec![],
            clashes: LazyStore::default(),
            shared: lock.shared.clone(),
            parent,
            func_counter: lock.func_counter.clone(),
            class_counter: lock.class_counter.clone(),
            level
        }));
        lock.subtables.push(sub_table.clone());
        sub_table
    }

    fn get_highest_table(&self) -> Self {
        let lock = self.lock();
        if let Some(parent) = &lock.parent {
            return parent.get_highest_table();
        }
        self.clone()
    }
}
