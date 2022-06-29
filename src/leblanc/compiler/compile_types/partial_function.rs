use crate::{LeBlancType, TypedToken};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;

#[derive(Eq, Debug, Clone, Hash)]
pub struct PartialFunction {
    pub name: String,
    pub args: Vec<LeBlancArgument>,
    pub returns: Vec<LeBlancType>
}

impl PartialFunction {
    pub fn from_token_args(token: &TypedToken) -> PartialFunction {
        return PartialFunction {
            name: token.as_string(),
            args: LeBlancArgument::from_positional(&token.typing()[0].clone()),
            returns: vec![]
        }
    }

    pub fn from_token_returns(token: &TypedToken) -> PartialFunction {
        return PartialFunction {
            name: token.as_string(),
            args: vec![],
            returns: token.typing()[1].clone()
        }
    }

    pub fn from_method(method: Method, returns: Vec<LeBlancType>) -> PartialFunction {
        return PartialFunction::from_method_store(method.store(), returns)
    }

    pub fn from_method_store(method_store: &MethodStore, returns: Vec<LeBlancType>) -> PartialFunction {
        PartialFunction {
            name: method_store.name.clone(),
            args: method_store.arguments.clone(),
            returns,
        }
    }
}

impl PartialEq for PartialFunction {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name { return false; }


        let max_self_args = match self.args.last() {
            Some(item) => item.position + 1,
            None => 0
        };
        let max_other_args = match other.args.last() {
            Some(item) => item.position + 1,
            None => 0
        };

        let mut main_iter = self.args.clone();
        let mut other_iter = other.args.clone();
        let (max, mut main_iter, mut sub_iter) = if max_self_args > max_other_args {
            for _ in 0..(max_self_args-max_other_args) as usize { other_iter.push(LeBlancArgument::null((other_iter.len()) as u32))}
            (max_self_args, self.args.iter(), other_iter.iter())
        } else {
            for _ in 0..(max_other_args-max_self_args) as usize { main_iter.push(LeBlancArgument::null((main_iter.len()) as u32))}
            (max_other_args, other.args.iter(), main_iter.iter())
        };
        for i in 0..max {
            if !main_iter.any(|arg| arg.position == (i as u32) && sub_iter.any(|o| o == arg)) {
                println!("False");
                return false;
            }
            println!("True");
        }
        true
    }
}