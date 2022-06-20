use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Debug, Eq)]
pub struct PartialToken {
    token: String,
    pub lang_type: CompileVocab
}

impl PartialToken {
    pub fn from(token: &TypedToken) -> PartialToken {
        return PartialToken {
            token: token.as_string(),
            lang_type: token.lang_type()
        }
    }
}

impl PartialEq for PartialToken {
    fn eq(&self, other: &Self) -> bool {
        if self.token != other.token { return false; }
        if self.lang_type.to_string() == other.lang_type.to_string() { return true; }
        return self.lang_type.matches("function") && other.lang_type.matches("function")
    }
}

impl Hash for PartialToken {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token.hash(state);
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct TypedToken{
    base: Token,
    lang_type: CompileVocab,
    scope: i32,
    a_typing: Vec<Vec<LeBlancType>>,
    global: bool
}

// line-number | char | symbol-number... | lang_type (0-255) | LeBlancType (0-255) . u32 | amount | LeBlancType


impl TypedToken {
    pub fn new(token: Token, vocab: CompileVocab, scope: i32, global: bool) -> TypedToken {
        return TypedToken {
            base: token,
            lang_type: vocab,
            scope,
            a_typing: vec![vec![], vec![]],
            global
        }
    }

    pub fn empty() -> TypedToken {
        return TypedToken {
            base: Token::empty(),
            lang_type: CompileVocab::UNKNOWN(LeBlancType::Class(0)),
            scope: -1,
            a_typing: vec![vec![], vec![]],
            global: false
        }
    }

    pub fn lang_type(&self) -> CompileVocab { self.lang_type }

    pub fn scope(&self) -> i32 { self.scope }

    pub fn token(&self) -> &Token { &self.base }

    pub fn typing(&self) -> &Vec<Vec<LeBlancType>> { &self.a_typing }

    pub fn typing_mut(&mut self) -> &mut Vec<Vec<LeBlancType>> { &mut self.a_typing }

    pub fn global(&self) -> bool { self.global }

    pub fn as_string(&self) -> String { self.base.as_string() }

    pub fn set_type(&mut self, vocab: CompileVocab) {
        self.lang_type = vocab;
    }

    pub fn set_scope(&mut self, scope: i32) { self.scope = scope; }

    pub fn set_typing_args(&mut self, typing: &mut Vec<LeBlancType>) {
        self.a_typing[0].append( typing) }

    pub fn set_typing_returns(&mut self, typing: Vec<LeBlancType>) { self.a_typing[1] = typing }

    pub fn set_typing(&mut self, typing: Vec<Vec<LeBlancType>>) { self.a_typing = typing; }

    pub fn set_global(&mut self, global: bool) { self.global = global }

    pub fn as_partial(&self) -> PartialToken { PartialToken::from(self) }

    pub fn as_stub_string(&self) -> String {
        let stub_string =  &self.base.line_number().to_string();
        let symbol_string = self.base.first_symbol_or_empty().symbol_number().to_string() + "|" + &self.as_string();
        let comp_type_string = self.lang_type.to_string();
        let scope_string = self.scope.to_string();
        let global_string = if self.global {
            "1".to_string()
        } else {
            "0".to_string()
        };
        let typings = self.a_typing.iter().map(|t| t[1].to_string() + "|").collect::<String>();



        return stub_string.to_owned() + "|" + &symbol_string.len().to_string() + "|"  + &symbol_string + &comp_type_string + "|" + &scope_string + "|" + &global_string + "|" + &typings + "&&";
    }
}

impl Display for TypedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base.as_string())
    }
}

impl Clone for TypedToken {
    fn clone(&self) -> Self {
        return TypedToken {
            base: self.base.copy(),
            lang_type: self.lang_type.clone(),
            scope: self.scope,
            a_typing: self.a_typing.clone(),
            global: self.global
        }
    }
}

