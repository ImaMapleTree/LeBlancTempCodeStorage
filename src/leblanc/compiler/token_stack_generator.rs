use crate::CompileVocab::FUNCTION;
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, CompileVocab, FunctionType, Specials};
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator;
use crate::leblanc::rustblanc::relationship::{Node, to_node_vec};

pub fn create_stack<'a>(tokens: &mut Vec<Node<TypedToken>>, stack: &'a mut Vec<TypedToken>) -> &'a mut Vec<TypedToken> {
    println!("Recursively called");
    while !tokens.is_empty() {
        println!("Tokens: {:?}", tokens.iter().map(|i| i.value.to_string()).collect::<Vec<String>>());
        let peek_token = &tokens.last().unwrap().value;
        let mut prime_token = peek_token;
        let mut marker = tokens.len()-1;
        for i in (0..tokens.len()).rev() {
            let comp_token = &tokens.get(i).unwrap().value;
            if comp_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::Comma) { break; }
            if comp_token.lang_type().priority() < prime_token.lang_type().priority() {
                prime_token = comp_token;
                marker = i;
            }
        }

        let consumed = tokens.remove(marker);
        //println!("Consumed: {}", &consumed.value);
        if consumed.value.lang_type() == CompileVocab::BOUNDARY(BoundaryType::Semicolon) {
            //continue;
        }

        if consumed.value.lang_type() == CompileVocab::BOUNDARY(BoundaryType::ParenthesisOpen) {
            create_stack(&mut to_node_vec(&consumed.children), stack);
        } else if consumed.value.lang_type() == CompileVocab::BOUNDARY(BoundaryType::BracketOpen) {
            create_stack(&mut to_node_vec(&consumed.children), stack);
            stack.push(consumed.value.clone());
        }
        else if let CompileVocab::KEYWORD(keyword) = consumed.value.lang_type() {
            stack.push(consumed.value.clone());
            create_stack(tokens, stack);
            if keyword == LBKeyword::While {
                let stack_top = stack.remove(0);
                let stack_bottom = stack.pop().unwrap();
                stack.insert(0, stack_bottom);
                stack.push(stack_top);
            }
        }
        else {
            stack.push(consumed.value.clone());
            let _stack_length = stack.len();
            if consumed.value.lang_type().matches("operator") {
                if consumed.value.lang_type().priority() <= 5 {
                    create_stack(&mut tokens.drain(0..marker).into_iter().collect(), stack);
                    create_stack(tokens, stack);
                } else if consumed.value.lang_type() == CompileVocab::OPERATOR(LBOperator::QuickList) {
                    /*println!("Increment Tokens: {:?}", tokens.iter().map(|i| i.value.to_string()).collect::<Vec<String>>());
                    if tokens[tokens.len()-1].value.lang_type().matches("boundary") {
                        other_token_stack.push(tokens.pop().unwrap());
                    }*/
                    let mut other_token_stack: Vec<Node<TypedToken>> = tokens.drain(0..marker).into_iter().collect();
                    create_stack(tokens, stack);
                    create_stack(&mut other_token_stack, stack);
                    //println!("Token at current length: {:?}", stack[length+1]);
                    //let iterator_setup = stack.pop().unwrap();
                } else if consumed.value.lang_type() == CompileVocab::OPERATOR(LBOperator::Index) {
                    create_stack(&mut tokens.drain(0..marker).into_iter().collect(), stack);
                    create_stack(tokens, stack);
                    create_stack(&mut to_node_vec(&consumed.children), stack);
                } else if consumed.value.lang_type() == CompileVocab::OPERATOR(LBOperator::Increment) {

                }
                else {
                    create_stack(&mut tokens.drain(0..marker).into_iter().collect(), stack);
                    create_stack(tokens, stack);
                }
            }
            else if consumed.value.lang_type().matches("function") && consumed.value.lang_type() != FUNCTION(FunctionType::Reference){
                let mut node_vec = to_node_vec(&tokens.remove(marker).children);
                create_stack(&mut node_vec, stack);
            }
            else if consumed.value.lang_type() == CompileVocab::SPECIAL(Specials::StackAppend, 0) {
                let mut other_token_stack = tokens.drain(0..marker).into_iter().collect();
                create_stack(tokens, stack);
                create_stack(&mut other_token_stack, stack);
            }
        }





    }
    println!("Recursively exited");
    stack



}