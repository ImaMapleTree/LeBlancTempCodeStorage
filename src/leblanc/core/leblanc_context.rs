use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableContext {
    pub name: String,
    pub line_number: u32,
    pub file: String,
    pub state: VariableState
}

impl VariableContext {
    pub fn empty() -> VariableContext {
        return VariableContext {
            name: "".to_string(),
            line_number: 0,
            file: "".to_string(),
            state: VariableState::Stack
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

