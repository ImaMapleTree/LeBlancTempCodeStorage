use std::fmt::{Debug, Formatter};
use crate::leblanc::compiler::symbols::SymbolType::{Alphabetic, ControlCharacter, Digit, Unknown, Whitespace};

#[derive(Copy, Clone, Eq, Hash)]
pub struct Symbol {
    character: char,
    pub pre_whitespace: bool,
    pub post_whitespace: bool,
    pub is_start_quote: bool,
    pub is_end_quote: bool,
    pub symbol_type: SymbolType,
    symbol_number: u32,
    line_number: u32
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum SymbolType {
    Alphabetic,
    Digit,
    Whitespace,
    ControlCharacter,
    Unknown
}

impl SymbolType {
    pub fn of(character: char) -> SymbolType {
        if character.is_whitespace() { Whitespace }
        else if character.is_ascii_digit() { Digit }
        else if character.is_alphabetic() {Alphabetic}
        else if character.is_ascii_punctuation() { ControlCharacter }
        else { Unknown }
    }
}

impl Symbol {
    pub fn new(character: char, pre_whitespace: bool, post_whitespace: bool, is_start_quote: bool, is_end_quote: bool,
               symbol_type: SymbolType, symbol_number: u32, line_number: u32) -> Symbol {
        Symbol {
            character, pre_whitespace, post_whitespace, is_start_quote, is_end_quote, symbol_type, symbol_number, line_number
        }
    }

    pub fn get_type(&self) -> &SymbolType {&self.symbol_type}

    pub fn line_number(&self) -> u32 {self.line_number}

    pub fn symbol_number(&self) -> u32 {self.symbol_number}

    pub fn char(&self) -> &char {&self.character}

    pub fn empty() -> Symbol {
        Symbol {
            character: '\0',
            pre_whitespace: false,
            post_whitespace: false,
            is_start_quote: false,
            is_end_quote: false,
            symbol_type: Unknown,
            symbol_number: 0,
            line_number: 0
        }
    }

    pub fn as_string(&self) -> String {
        return self.char().to_string();
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        return self.char() == other.char();
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}