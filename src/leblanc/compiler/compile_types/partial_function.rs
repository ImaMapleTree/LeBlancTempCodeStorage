use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Eq, Debug, Clone, Hash)]
pub struct PartialFunction {
    pub name: String,
    pub args: Vec<LeBlancArgument>,
    pub returns: Vec<LeBlancType>
}

impl PartialFunction {
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
        let (max, main_iter, other_iter) = if max_self_args > max_other_args {
            for _ in 0..(max_self_args-max_other_args) as usize { other_iter.push(LeBlancArgument::null((other_iter.len()) as u32))}
            (max_self_args, self.args.clone(), other_iter)
        } else {
            for _ in 0..(max_other_args-max_self_args) as usize { main_iter.push(LeBlancArgument::null((main_iter.len()) as u32))}
            (max_other_args, other.args.clone(), main_iter)
        };
        for i in 0..max {
            if !main_iter.iter().any(|arg| arg.position == (i as u32) && other_iter.iter().any(|o| o == arg)) {
                return false;
            }
        }
        true
    }
}