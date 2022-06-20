use crate::{LeBlancType, TypedToken};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PartialFunction {
    name: String,
    pub args: Vec<LeBlancType>
}

impl PartialFunction {
    pub fn from_token_args(token: &TypedToken) -> PartialFunction {
        return PartialFunction {
            name: token.as_string(),
            args: token.typing()[0].clone()
        }
    }

    pub fn from_token_returns(token: &TypedToken) -> PartialFunction {
        return PartialFunction {
            name: token.as_string(),
            args: token.typing()[1].clone()
        }
    }

    pub fn from_method(method: Method) -> PartialFunction {
        return PartialFunction::from_method_store(method.store())
    }

    pub fn from_method_store(method_store: &MethodStore) -> PartialFunction {
        return PartialFunction {
            name: method_store.name.clone(),
            args: method_store.arguments.iter().map(|arg| arg.typing).collect()
        }
    }
}