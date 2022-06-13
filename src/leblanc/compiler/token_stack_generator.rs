use std::process::Child;
use std::sync::Arc;
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, CompileVocab};
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::relationship::{child_adapter, Node, NodeData, to_node_vec};

pub fn create_stack<'a>(mut tokens: &mut Vec<Node<TypedToken>>, stack: &'a mut Vec<TypedToken>) -> &'a mut Vec<TypedToken> {
    while !tokens.is_empty() {
        let peek_token = &tokens.get(0).unwrap().value;
        let mut prime_token = peek_token;
        let mut marker = 0;
        for i in (0..tokens.len()).rev() {
            let comp_token = &tokens.get(i).unwrap().value;
            if comp_token.lang_type().priority() <= prime_token.lang_type().priority() {
                prime_token = comp_token;
                marker = i;
            }
        }

        let mut consumed = tokens.remove(marker);
        if consumed.value.lang_type() == CompileVocab::BOUNDARY(BoundaryType::Semicolon) {
            continue;
        }

        if consumed.value.lang_type() == CompileVocab::BOUNDARY(BoundaryType::ParenthesisOpen) {
            create_stack(&mut to_node_vec(&consumed.children), stack);
        }
        else if let CompileVocab::KEYWORD(keyword) = consumed.value.lang_type() {
            stack.push(consumed.value.clone());
            create_stack(tokens, stack);
        }
        else {
            stack.push(consumed.value.clone());
            if consumed.value.lang_type().matches("operator") {

            }
            else if consumed.value.lang_type().matches("function") {
                let mut new_tokens = Vec::new();
                new_tokens.push(tokens.remove(marker));
                create_stack(&mut new_tokens, stack);
            }
        }





    }
    return stack;



}