use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::Appendable;

#[derive(Eq, PartialEq, Clone, Debug, PartialOrd)]
pub struct LeBlancArgument {
    pub typing: LeBlancType,
    pub position: u32,
    pub keyword: bool,
    pub variable: bool
}

impl LeBlancArgument {
    pub fn default(typing: LeBlancType, position: u32) -> LeBlancArgument {
        return LeBlancArgument {
            typing,
            position,
            keyword: false,
            variable: false
        }
    }

    pub fn from_positional(args: &[LeBlancType]) -> Vec<LeBlancArgument> {
        let mut return_args = vec![];
        for lbtype in args {
            return_args.append_item( LeBlancArgument::default(*lbtype, return_args.len() as u32));
        }
        return return_args;
    }
}


pub fn number_argset() -> Vec<LeBlancArgument> {
    let mut args = Vec::new();
    args.append_item(LeBlancArgument::default(LeBlancType::Short, 0));
    args.append_item(LeBlancArgument::default(LeBlancType::Int, 0));
    args.append_item(LeBlancArgument::default(LeBlancType::Int64, 0));
    args.append_item(LeBlancArgument::default(LeBlancType::Int128, 0));
    args.append_item(LeBlancArgument::default(LeBlancType::Arch, 0));
    return args;
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
        self.variable.hash(state);
        self.position.hash(state);
    }
}