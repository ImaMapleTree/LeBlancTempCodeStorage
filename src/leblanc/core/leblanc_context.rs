use std::fmt;

#[derive(Debug, Hash, PartialEq, PartialOrd)]
pub struct VariableContext {
    pub name: String,
    pub line_number: u32,
    pub file: String,
    pub state: VariableState,
    pub relationship: u32
}

impl VariableContext {
    pub fn empty() -> VariableContext {
        return VariableContext {
            name: "".to_string(),
            line_number: 0,
            file: "".to_string(),
            state: VariableState::Stack,
            relationship: 0
        }
    }

    pub fn shell(name: String, relationship: u32) -> VariableContext {
        return VariableContext {
            name,
            line_number: 0,
            file: "".to_string(),
            state: VariableState::Local,
            relationship
        }
    }
}

impl Clone for VariableContext {
    fn clone(&self) -> Self {
        return VariableContext {
            name: self.name.clone(),
            line_number: self.line_number,
            file: self.file.clone(),
            state: self.state,
            relationship: self.relationship
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd)]
pub enum VariableState {
    Global,
    Local,
    Stack
}

impl fmt::Display for VariableState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

