use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::Appendable;
use crate::LeBlancType::Null;

#[derive(Eq, Clone, Debug, PartialOrd, Copy)]
pub struct LeBlancArgument {
    pub typing: LeBlancType,
    pub position: u32,
    pub required: bool,
    pub keyword: bool,
    pub variable: bool
}

impl LeBlancArgument {
    pub fn default(typing: LeBlancType, position: u32) -> LeBlancArgument {
        LeBlancArgument {
            typing,
            position,
            required: true,
            keyword: false,
            variable: false
        }
    }

    pub fn optional(typing: LeBlancType, position: u32) -> LeBlancArgument {
        LeBlancArgument {
            typing,
            position,
            required: false,
            keyword: false,
            variable: false
        }
    }

    pub fn variable(typing: LeBlancType, position: u32) -> LeBlancArgument {
        LeBlancArgument {
            typing,
            position,
            required: false,
            keyword: false,
            variable: true
        }
    }

    pub fn from_positional(args: &[LeBlancType]) -> Vec<LeBlancArgument> {
        let mut return_args = vec![];
        for lbtype in args {
            return_args.append_item( LeBlancArgument::default(*lbtype, return_args.len() as u32));
        }
        return_args
    }

    pub fn null(position: u32) -> LeBlancArgument {
        LeBlancArgument {
            typing: LeBlancType::Null,
            position,
            required: false,
            keyword: false,
            variable: false
        }
    }
}


pub fn number_argset(position: u32) -> Vec<LeBlancArgument> {
    let mut args = Vec::new();
    args.append_item(LeBlancArgument::default(LeBlancType::Short, position));
    args.append_item(LeBlancArgument::default(LeBlancType::Int, position));
    args.append_item(LeBlancArgument::default(LeBlancType::Int64, position));
    args.append_item(LeBlancArgument::default(LeBlancType::Int128, position));
    args.append_item(LeBlancArgument::default(LeBlancType::Arch, position));
    args
}

impl Display for LeBlancArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.position, self.typing)
    }
}

impl Hash for LeBlancArgument {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        match self.typing {
            LeBlancType::Flex => {},
            _ => self.typing.hash(state)
        }
    }
}

impl PartialEq for LeBlancArgument {
    fn eq(&self, other: &Self) -> bool {
        if self.variable || other.variable {
            if self.typing == Null || other.typing == Null { return true }
            return self.typing == other.typing;
        }
        if self.position != other.position { return false; }
        if !self.required && (other.typing == LeBlancType::Null) { return true }
        if !other.required && (self.typing == LeBlancType::Null) { return true }
        self.typing == other.typing
    }
}