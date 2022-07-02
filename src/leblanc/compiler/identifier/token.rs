use std::fmt::{Display, Formatter};
use crate::leblanc::compiler::symbols::{Symbol, SymbolType};
use crate::leblanc::rustblanc::exception::leblanc_base_exception::LeblancBaseException;

#[derive(Debug, Clone, Hash)]
pub struct Token {
    symbols: Vec<Symbol>,
    line_number: u32
}

impl Token {
    pub fn empty() -> Token {
        Token {
            symbols: Vec::new(),
            line_number: 0
        }
    }

    pub fn from_string(string: String) -> Token {
        let mut symbols = vec![];
        for c in string.chars() {
            symbols.push(Symbol::new(c, false, false, false, false, SymbolType::of(c), 0, 0))
        }
        Token::new(symbols, 0)
    }

    pub fn from(symbol: Symbol) -> Token {
        let mut symbols = Vec::new();
        let line_number = symbol.line_number();
        symbols.push(symbol);

        Token {
            symbols,
            line_number
        }
    }

    pub fn new(symbols: Vec<Symbol>, line_number: u32) -> Token {
        Token {
            symbols,
            line_number
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.line_number = symbol.line_number();
        self.symbols.push(symbol);
    }

    pub fn insert_symbol(&mut self, index: usize, symbol: Symbol) {
        self.symbols.insert(index, symbol);
    }

    pub fn len(&self) -> usize {
        let size = self.symbols.len();

        if size == 0 || size > 1 {
            return size;
        }

        if *self.symbol(0).char() == '\0' {
            return 0;
        }

        size
    }

    pub fn copy(&self) -> Token {
        Token {
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
        self.symbol(0) 
    }

    pub fn last_symbol_or_empty(&self) -> Symbol {
        let length = self.symbols.len();
        if length == 0 {
            return Symbol::empty();
        }
        self.symbol(length-1)
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();
        for symbol in &self.symbols {
            string += String::from(*symbol.char()).as_str();
        }
        string
    }

    pub fn symbols(&self) -> &Vec<Symbol> { &self.symbols }

    pub fn line_number(&self) -> u32 { self.line_number }

    pub fn set_line_number(&mut self, line_number: u32) {
        self.line_number = line_number;
    }
}
impl Eq for Token {}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.symbols == other.symbols
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Line Number: {})", self.as_string(), self.line_number)
    }
}