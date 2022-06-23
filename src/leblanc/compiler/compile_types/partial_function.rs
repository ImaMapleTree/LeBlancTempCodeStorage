use crate::{LeBlancType, TypedToken};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;

#[derive(Eq, Debug, Clone, Hash)]
pub struct PartialFunction {
    pub name: String,
    pub args: Vec<LeBlancArgument>
}

impl PartialFunction {
    pub fn from_token_args(token: &TypedToken) -> PartialFunction {
        return PartialFunction {
            name: token.as_string(),
            args: LeBlancArgument::from_positional(&token.typing()[0].clone())
        }
    }

    pub fn from_token_returns(token: &TypedToken) -> PartialFunction {
        return PartialFunction {
            name: token.as_string(),
            args: LeBlancArgument::from_positional(&token.typing()[1].clone())
        }
    }

    pub fn from_method(method: Method) -> PartialFunction {
        return PartialFunction::from_method_store(method.store())
    }

    pub fn from_method_store(method_store: &MethodStore) -> PartialFunction {
        return PartialFunction {
            name: method_store.name.clone(),
            args: method_store.arguments.clone()
        }
    }
}

impl PartialEq for PartialFunction {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name { return false; }
        let max = if self.args.len() > other.args.len() {
            self.args.len()
        } else { other.args.len() };
        for i in 0..max {
            let self_arg = *self.args.get(i).unwrap_or(&LeBlancArgument::null(i as u32));
            let other_arg = *other.args.get(i).unwrap_or(&LeBlancArgument::null(i as u32));
            println!("Arg {}: {:?} vs {:?}", i, self_arg, other_arg);
            if self_arg != other_arg  {
                return false;
            }
        }
        return true;
    }
}