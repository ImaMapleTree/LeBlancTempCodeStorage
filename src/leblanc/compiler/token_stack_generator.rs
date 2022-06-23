use crate::CompileVocab::FUNCTION;
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, CompileVocab, FunctionType, Specials};
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator;
use crate::leblanc::rustblanc::relationship::{Node, to_node_vec};

pub fn create_stack<'a>(mut tokens: &mut Vec<Node<TypedToken>>, stack: &'a mut Vec<TypedToken>) -> &'a mut Vec<TypedToken> {
    //println!("Tokens: {:?}", tokens.iter().map(|i| i.value.to_string()).collect::<Vec<String>>());
    while !tokens.is_empty() {
        let peek_token = &tokens.get(0).unwrap().value;
        let mut prime_token = peek_token;
        let mut marker = 0;
        for i in (0..tokens.len()).rev() {
            let comp_token = &tokens.get(i).unwrap().value;
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
        }
        else if let CompileVocab::KEYWORD(_keyword) = consumed.value.lang_type() {
            stack.push(consumed.value.clone());
            create_stack(tokens, stack);
        }
        else {
            stack.push(consumed.value.clone());
            let _stack_length = stack.len();
            if consumed.value.lang_type().matches("operator") {
                if consumed.value.lang_type().priority() <= 5 {
                    create_stack(&mut tokens.drain(0..marker).into_iter().collect(), stack);
                    create_stack(&mut tokens, stack);
                } else {
                    if consumed.value.lang_type() == CompileVocab::OPERATOR(LBOperator::Increment) {
                        //println!("Tokens: {:?}", tokens.iter().map(|i| i.value.to_string()).collect::<Vec<String>>());
                        let mut other_token_stack: Vec<Node<TypedToken>> = tokens.drain(0..marker).into_iter().collect();
                        if tokens[tokens.len()-1].value.lang_type().matches("boundary") {
                            other_token_stack.push(tokens.pop().unwrap());
                        }
                        create_stack(&mut tokens, stack);
                        create_stack(&mut other_token_stack, stack);
                    }
                    //let element = stack.pop().unwrap();
                    //println!("Stolen element: {}", element);
                    //stack.insert(stack_length+1, element);
                }
            }
            else if consumed.value.lang_type().matches("function") && consumed.value.lang_type() != FUNCTION(FunctionType::Reference){
                let mut node_vec = to_node_vec(&tokens.remove(marker).children);
                create_stack(&mut node_vec, stack);
            }
            else if consumed.value.lang_type() == CompileVocab::SPECIAL(Specials::StackAppend, 0) {
                let mut other_token_stack = tokens.drain(0..marker).into_iter().collect();
                create_stack(&mut tokens, stack);
                create_stack(&mut other_token_stack, stack);
            }
        }





    }
    return stack;



}