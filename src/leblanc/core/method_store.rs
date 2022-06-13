use crate::leblanc::core::leblanc_argument::LeBlancArgument;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct MethodStore {
    pub name: String,
    pub arguments: Vec<LeBlancArgument>,
}

impl MethodStore {
    pub fn no_args(name: String) -> MethodStore {
        return MethodStore {
            name,
            arguments: vec![],
        }
    }

    pub fn new(name: String, arguments: Vec<LeBlancArgument>) -> MethodStore {
        return MethodStore {
            name,
            arguments,
        }
    }
}