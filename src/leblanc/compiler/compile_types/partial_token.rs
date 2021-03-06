use std::hash::{Hash, Hasher};
use crate::{CompileVocab, TypedToken};

#[derive(Debug, Eq)]
pub struct PartialToken {
    token: String,
    pub lang_type: CompileVocab
}

impl PartialToken {
    pub fn new(string: String, lang_type: CompileVocab) -> PartialToken {
        PartialToken {
            token: string,
            lang_type
        }
    }

    pub fn from(token: &TypedToken) -> PartialToken {
        PartialToken {
            token: token.as_string(),
            lang_type: token.lang_type()
        }
    }
}

impl PartialEq for PartialToken {
    fn eq(&self, other: &Self) -> bool {
        if self.token != other.token { return false; }
        if self.lang_type.to_string() == other.lang_type.to_string() { return true; }
        self.lang_type.matches("function") && other.lang_type.matches("function")
    }
}

impl Hash for PartialToken {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token.hash(state);
    }
}