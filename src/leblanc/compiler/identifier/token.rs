use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use crate::leblanc::compiler::symbols::Symbol;
use crate::leblanc::rustblanc::exception::leblanc_base_exception::LeblancBaseException;

#[derive(Debug, Clone, Hash)]
pub struct Token {
    symbols: Vec<Symbol>,
    line_number: u32
}

impl Token {
    pub fn empty() -> Token {
        return Token {
            symbols: Vec::new(),
            line_number: 0
        };
    }

    pub fn from(symbol: Symbol) -> Token {
        let mut symbols = Vec::new();
        let line_number = symbol.line_number();
        symbols.push(symbol);

        return Token {
            symbols,
            line_number
        }
    }

    pub fn new(symbols: Vec<Symbol>, line_number: u32) -> Token {
        return Token {
            symbols,
            line_number
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.line_number = symbol.line_number();
        self.symbols.push(symbol);
    }

    pub fn len(&self) -> usize {
        let size = self.symbols.len();

        if size == 0 || size > 1 {
            return size;
        }

        if *self.symbol(0).char() == '\0' {
            return 0;
        }

        return size;
    }

    pub fn copy(&self) -> Token {
        return Token {
            symbols: self.symbols.clone(),
            line_number: self.line_number
        }
    }

    pub fn symbol(&self, index: usize) -> Symbol {
        return match self.symbols.get(index) {
            None => {
                LeblancBaseException::new(&format!("Error indexing symbol in parser ({} not in array of size {})", index, self.symbols.len())
                                          , true, 5009002).handle().unwrap();
                Symbol::empty()
            }
            Some(sym) => *sym
        }
    }

    pub fn first_symbol_or_empty(&self) -> Symbol {
        let length = self.symbols.len();
        if length == 0 {
            return Symbol::empty();
        }
        return self.symbol(0) 
    }

    pub fn last_symbol_or_empty(&self) -> Symbol {
        let length = self.symbols.len();
        if length == 0 {
            return Symbol::empty();
        }
        return self.symbol(length-1)
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();
        for symbol in &self.symbols {
            string += String::from(*symbol.char()).as_str();
        }
        return string;
    }

    pub fn symbols(&self) -> &Vec<Symbol> { &self.symbols }

    pub fn line_number(&self) -> u32 { self.line_number }
}
impl Eq for Token {}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.symbols == other.symbols;
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Line Number: {})", self.as_string(), self.line_number)
    }
}