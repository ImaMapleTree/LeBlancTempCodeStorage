use crate::leblanc::compiler::grammar::ast::ParseType;
use crate::leblanc::rustblanc::copystring::CopyStringable;
use crate::LeBlancType;

pub fn parse_type_to_lb_type(t: ParseType) -> LeBlancType {
    match t {
        ParseType::Flex => LeBlancType::Flex,
        ParseType::String => LeBlancType::String,
        ParseType::Int => LeBlancType::Int,
        ParseType::Float => LeBlancType::Float,
        ParseType::Double => LeBlancType::Double,
        ParseType::Function => LeBlancType::Function,
        ParseType::Group => LeBlancType::Group,
        ParseType::Promise => LeBlancType::Promise,
        ParseType::SelfRef => LeBlancType::SelfType,
        ParseType::SuperLambda => LeBlancType::SuperLambda,
        ParseType::Class(str) => LeBlancType::Class(str.to_cstring()),
        ParseType::Trait(str, bool) => LeBlancType::Trait(str.to_cstring(), bool)
    }
}