use crate::leblanc::compiler3::leblanc_type::LBType;
use crate::leblanc::compiler3::symbols::lower_symbols::{STSymbol};
use crate::leblanc::compiler::parser::ast::Location;
use crate::leblanc::rustblanc::copystring::CopyString;


//TODO: custom code assertions so that you can keep your code regular and prevent compiling unless the assertion has been met
#[derive(Default)]
pub struct SymbolTable {
    pub(crate) functions: Vec<STFunction>,
    pub(crate) classes: Vec<STClass>,
    pub(crate) traits: Vec<STrait>,
    pub(crate) extensions: Vec<STExtension>,
    pub(crate) enums: Vec<STEnumSet>,
    pub(crate) globals: Vec<STProperty>
}

pub struct STClass {
    pub(crate) ident: CopyString,
    pub(crate) id: usize,
    pub(crate) traits: Vec<STrait>,
    pub(crate) properties: Vec<STProperty>,
    pub(crate) functions: Vec<STFunction>,
    pub(crate) location: Location
}

pub struct STrait {
    pub(crate) ident: CopyString,
    pub(crate) id: usize,
    pub(crate) supertraits: Vec<STrait>,
    pub(crate) functions: Vec<STFunction>,
    pub(crate) auto_functions: Vec<STFunction>,
    pub(crate) properties: Vec<STProperty>,
}

pub struct STFunction {
    pub(crate) ident: CopyString,
    pub(crate) id: usize,
    pub(crate) args: Vec<STIdent>,
    pub(crate) returns: Vec<SType>,
    pub(crate) body: Vec<STCode>,
}

pub struct STExtension {
    pub(crate) ident: CopyString,
    pub(crate) id: usize,
    pub(crate) classes: Vec<STIdent>,
    pub(crate) properties: Vec<STProperty>,
    pub(crate) functions: Vec<STFunction>,
}

pub struct STEnumSet {
    pub(crate) ident: CopyString,
    pub(crate) id: usize,
    pub(crate) items: Vec<STEnumItem>
}

pub struct STEnumItem {
    pub(crate) ident: CopyString,
    pub(crate) fields: Vec<SType>
}

pub enum STProperty {
    Property(STIdent, Option<Vec<STCode>>),
    Casual(STIdent, Option<STSymbol>),
}

pub struct STIdent {
    pub(crate) ident: CopyString,
    pub(crate) typ: Option<SType>
}

pub enum SType {
    Resolved(LBType),
    Unresolved(Option<i64>)
}

pub struct STCode {
    pub(crate) idents: Vec<STIdent>,
    pub(crate) lower: Vec<STSymbol>,
    pub(crate) line: usize
}

