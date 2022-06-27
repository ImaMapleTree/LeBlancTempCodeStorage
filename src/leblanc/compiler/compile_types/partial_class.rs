use crate::leblanc::compiler::compile_types::partial_function::PartialFunction;
use crate::leblanc::compiler::compile_types::partial_token::PartialToken;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct PartialClass {
    methods: Vec<PartialFunction>,
    members: Vec<PartialToken>,
}

impl PartialClass {
    pub fn new(methods: Vec<PartialFunction>, members: Vec<PartialToken>) -> PartialClass {
        PartialClass {
            methods,
            members
        }
    }

    pub fn empty() -> PartialClass {
        PartialClass {
            methods: vec![],
            members: vec![]
        }
    }
}