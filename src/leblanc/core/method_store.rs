use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method_tag::MethodTag;

#[derive(Eq, Hash, PartialEq, Debug, Clone, PartialOrd)]
pub struct MethodStore {
    pub name: String,
    pub arguments: Vec<LeBlancArgument>,
}

impl MethodStore {
    pub fn no_args(name: String) -> MethodStore {
        MethodStore {
            name,
            arguments: vec![],
        }
    }

    pub fn new(name: String, arguments: Vec<LeBlancArgument>) -> MethodStore {
        MethodStore {
            name,
            arguments,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MethodShell {
    pub context: MethodStore,
    pub tags: BTreeSet<MethodTag>
}


impl MethodShell {
    pub fn new(context: MethodStore, tags: BTreeSet<MethodTag>) -> MethodShell {
        MethodShell {
            context,
            tags
        }
    }
}

//noinspection RsExternalLinter
impl Hash for MethodShell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.hash(state);
        self.tags.hash(state);
    }
}