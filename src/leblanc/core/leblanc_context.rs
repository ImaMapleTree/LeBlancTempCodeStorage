use std::fmt;

use crate::leblanc::rustblanc::copystring::CopyString;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Copy, Default)]
pub struct VariableContext {
    pub name: CopyString,
    pub line_number: u32,
    pub file: CopyString,
    pub state: VariableState,
    pub relationship: u32
}

impl VariableContext {
    pub fn empty() -> VariableContext {
        VariableContext {
            name: CopyString::default(),
            line_number: 0,
            file: CopyString::default(),
            state: VariableState::Stack,
            relationship: 0
        }
    }

    pub fn shell(name: String, relationship: u32) -> VariableContext {
        VariableContext {
            name: CopyString::new(name),
            line_number: 0,
            file: CopyString::default(),
            state: VariableState::Local,
            relationship
        }
    }
}

impl Clone for VariableContext {
    fn clone(&self) -> Self {
        VariableContext {
            name: self.name,
            line_number: self.line_number,
            file: self.file,
            state: self.state,
            relationship: self.relationship
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Default)]
pub enum VariableState {
    Global,
    Local,
    #[default]
    Stack
}

impl fmt::Display for VariableState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

