use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use parking_lot::Mutex;
use crate::leblanc::compiler3::leblanc_type::LBType;
use crate::leblanc::compiler3::symbols::symbol_table::Scope::Global;
use crate::leblanc::compiler::parser::ast::Location;
use crate::leblanc::core::interpreter::instructions::Instruction;
use crate::leblanc::rustblanc::copystring::CopyString;

pub type LinkedSymbolTable = Arc<Mutex<SymbolTable>>;

pub trait SubTable {
    fn create_sub_table(&mut self) -> Self;
    fn get_highest_table(&self) -> Self;
}



#[derive(Clone, Debug)]
pub struct SymbolTable {
    pub(crate) intention: TableIntention,
    pub(crate) symbols: Vec<Symbol>,
    pub(crate) parent: Option<LinkedSymbolTable>,
    pub(crate) linked: Vec<LinkedSymbolTable>,
    pub(crate) func_counter: Arc<AtomicU64>,
    pub(crate) class_counter: Arc<AtomicU64>,
    pub(crate) level: u16
}

#[derive(Copy, Clone, Eq, Debug)]
pub struct Symbol {
    pub(crate) name: CopyString,
    pub(crate) form: SymbolType,
    pub(crate) scope: Scope,
    pub(crate) typ: Option<LBType>,
    pub(crate) location: Option<Location>,
    pub(crate) instruction: u16
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SymbolType {
    Class(u64),
    Function(u64),
    Expression,
    Argument,
    Type,
    Ident,
    IdentAttribute
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Scope {
    Global,
    Intermediate,
    Local(u16), //65k scopes seems okay
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Eq)]
pub enum TableIntention {
    Assignment,
    MethodCall,
    #[default]
    None
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable {
            intention: Default::default(),
            symbols: vec![],
            parent: None,
            linked: vec![],
            func_counter: Arc::new(Default::default()),
            class_counter: Arc::new(AtomicU64::new(1)),
            level: 0
        }
    }
}


impl SymbolTable {
    /// Method used for tracking references, otherwise useless
    pub fn set_intention(&mut self, intention: TableIntention) {
        self.intention = intention;
    }

    pub fn add_symbol(&mut self, mut symbol: Symbol) {
        if self.level == 0 {
            symbol.scope = Global;
        }
        self.symbols.push(symbol);
        self.linked.iter_mut().for_each(|table| table.lock().symbols.push(symbol));
    }

    pub fn add_generic_symbol(&mut self, name: CopyString, instruction: u16, location: Option<Location>) {
        self.symbols.push(
            Symbol {
                name,
                form: SymbolType::Expression,
                scope: Scope::Local(self.level),
                typ: None,
                location,
                instruction
            }
        )
    }


    pub fn check_existence(&self, symbol: &Symbol) -> bool {
        println!("Self symbols: {:?}", self.symbols);
        if self.symbols.contains(symbol) { return true; }
        if let Some(parent) = &self.parent {
            return parent.lock().check_existence(symbol);
        }
        false
    }
}

impl SubTable for LinkedSymbolTable {
    fn create_sub_table(&mut self) -> Self {
        let mut lock = self.lock();
        let parent = Some(self.clone());
        let level = lock.level + 1;
        let sub_table = Arc::new(Mutex::new(SymbolTable {
            intention: Default::default(),
            symbols: Vec::new(),
            parent,
            linked: Vec::new(),
            func_counter: lock.func_counter.clone(),
            class_counter: lock.class_counter.clone(),
            level }));
        lock.linked.push(sub_table.clone());
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

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name { return false; }
        if self.form != other.form { return false; }
        if self.typ != other.typ { return false; }
        if self.instruction != other.instruction { return false; }
        true
    }
}