use std::collections::HashMap;
use crate::{BraceOpen, CompilationMode, CompileVocab, LeBlancType, Semicolon, TypedToken};
use crate::CompileVocab::{FUNCTION, UNKNOWN};
use crate::leblanc::compiler::compile_types::partial_token::PartialToken;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword::{Func, Returns, SelfRT};
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, FunctionType};
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{Comma, ParenthesisClosed};
use crate::leblanc::core::internal::methods::builtins::create_partial_functions;
use crate::leblanc::rustblanc::Appendable;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub::InvalidSyntax;
use crate::leblanc::rustblanc::relationship::Node;
use crate::LeBlancType::Class;

pub fn identify(mut typed_tokens: Vec<TypedToken>, mut import_tokens: Vec<Node<TypedToken>>, type_map: &mut HashMap<String, Vec<Vec<CompileVocab>>>, errors: &mut Vec<ErrorStub>, mode: CompilationMode) -> Vec<Node<TypedToken>>{

    identify_unknown(&mut typed_tokens, type_map);
    let mut func_matcher = identify_functions(&mut typed_tokens, errors);
    import_tokens.reverse();
    while !import_tokens.is_empty() {
        let token = &import_tokens.pop().unwrap().value;
        if token.lang_type() == FUNCTION(FunctionType::Header) {
            func_matcher.insert(token.as_partial(), token.typing()[1].clone());
        }
    }

    println!("{:#?}", func_matcher);
    for token in &mut typed_tokens {
        let mut partial_token = token.as_partial();
        let mut function_ref = false;
        if partial_token.lang_type == UNKNOWN(Class(0)) {
            partial_token.lang_type = FUNCTION(FunctionType::Call);
            function_ref = true;
        }
        if func_matcher.contains_key(&partial_token) {
            token.set_typing_returns(func_matcher.get(&partial_token).unwrap().clone());
            if function_ref { token.set_type(FUNCTION(FunctionType::Reference))}
        }
    }

    println!("Relationship");
    let length = typed_tokens.len();
    identify_caller_function_args(&mut typed_tokens, 0, length);
    for token in &typed_tokens {
        println!("Token: {:?}", token);
    }


    create_ownership(typed_tokens, mode)
}

pub fn identify_caller_function_args(typed_tokens: &mut Vec<TypedToken>, left_bound: usize, right_bound: usize) -> usize {
    let length = typed_tokens.len();
    let mut i_addition = 0;
    for mut i in left_bound..right_bound {
        i += i_addition;
        if typed_tokens.len() > i && typed_tokens[i].lang_type() == FUNCTION(FunctionType::Call) && length > i+1 {
            let mut can_add_type = true;
            let mut j_addition = 0;
            for mut j in i+1..length {
                /*println!("J token: {}", typed_tokens[j]);*/
                j += j_addition;
                if can_add_type && typed_tokens[j].lang_type() == FUNCTION(FunctionType::Call) {
                    let j_index = j;
                    j_addition = identify_caller_function_args(typed_tokens, j, 1+typed_tokens.iter().enumerate()
                        .filter(|&(index, token)| index > i && index > j+1 && token.lang_type() == CompileVocab::BOUNDARY(ParenthesisClosed)).map(|(index, _)| index).next().unwrap());
                    let mut j_token = typed_tokens[j_index].clone();
                    typed_tokens[i].set_typing_args(&mut j_token.typing_mut().pop().unwrap());
                    can_add_type = false;
                } else if can_add_type && typed_tokens[j].lang_type().stores_native_type() {
                    let j_type = *typed_tokens[j].lang_type().extract_native_type();
                    typed_tokens[i].set_typing_args(&mut vec![j_type]);
                    can_add_type = false;
                } else if typed_tokens[j].lang_type() == CompileVocab::FUNCTION(FunctionType::Reference) {
                    typed_tokens[i].set_typing_args(&mut vec![LeBlancType::Function]);
                    can_add_type = false;
                }
                else if typed_tokens[j].lang_type() == CompileVocab::BOUNDARY(Comma) {
                    can_add_type = true;
                }
                else if typed_tokens[j].lang_type() == CompileVocab::BOUNDARY(ParenthesisClosed) || typed_tokens[j].lang_type() == CompileVocab::BOUNDARY(Semicolon) {
                    i_addition += j_addition;
                    break;
                }

            }
        }
    }
    right_bound-left_bound
}

pub fn identify_functions(typed_tokens: &mut Vec<TypedToken>, errors: &mut Vec<ErrorStub>) -> HashMap<PartialToken, Vec<LeBlancType>>{
    let mut func_matcher: HashMap<PartialToken, Vec<LeBlancType>> = HashMap::new();
    create_partial_functions().iter().for_each(|p| {func_matcher.insert(PartialToken::new(p.name.clone(), FUNCTION(FunctionType::Header)), p.args.iter().map(|a| a.typing).collect::<Vec<LeBlancType>>());});
    let mut ndi = 0;
    let mut returns = false;
    let mut return_types = vec![];
    let mut arg_types = vec![];
    for i in 0..typed_tokens.len() {
        let token = &typed_tokens[i];
        if token.lang_type() == CompileVocab::KEYWORD(Func) {
            arg_types = vec![];
            if i+1 > typed_tokens.len() {
                println!("My error was here");
                errors.append_item( InvalidSyntax(token.clone()));
            } else {
                if ndi != 0 {
                    func_matcher.insert(typed_tokens[ndi].as_partial(), vec![Class(0)]);
                    typed_tokens[ndi].set_typing_returns(vec![Class(0)]);
                }
                ndi = i + 1;
            }
        }
        else if token.lang_type() == CompileVocab::KEYWORD(Returns) && ndi > 0 {
            returns = true;
        }
        else if returns {
            if token.lang_type() == CompileVocab::BOUNDARY(BraceOpen) {
                returns = false;
                func_matcher.insert(typed_tokens[ndi].as_partial(), return_types.clone());
                typed_tokens[ndi].set_typing_returns(return_types);
                typed_tokens[ndi].set_typing_args(&mut arg_types);
                return_types = vec![];
                ndi = 0;
            } else if token.lang_type() == CompileVocab::KEYWORD(SelfRT) {

            } else if let CompileVocab::TYPE(return_type) = token.lang_type() {
                return_types.append_item(return_type);
            } else if token.lang_type() != CompileVocab::BOUNDARY(Comma) {
                errors.append_item(InvalidSyntax(token.clone()));
            }
        }
        else if token.lang_type() == CompileVocab::KEYWORD(SelfRT) {
          if ndi > 0 {
              arg_types.push(LeBlancType::SelfType);
          }
        } else if let CompileVocab::TYPE(arg_type) = token.lang_type() {
            if ndi > 0 {
                arg_types.push(arg_type);
            }
        }


    }
    func_matcher
}

pub fn identify_unknown(typed_tokens: &mut Vec<TypedToken>, type_map: &mut HashMap<String, Vec<Vec<CompileVocab>>>) {
    for token in typed_tokens.iter_mut().filter(|t| t.lang_type().matches("unknown")) {
        if let Class(class_value) = token.lang_type().extract_native_type() {
            let optional_scopes = type_map.get_mut(&token.as_string());
            if optional_scopes.is_some() {
                let global_value = optional_scopes.as_ref().unwrap().get(0);
                if global_value.is_some() && !global_value.unwrap().is_empty() {
                    token.set_scope(0);
                    token.set_type(global_value.unwrap()[0]);
                } else if *class_value > 0 {
                    let class_scopes = optional_scopes.unwrap().get(*class_value as usize);
                    if class_scopes.is_some() && !class_scopes.unwrap().is_empty() {
                        token.set_scope(*class_value as i32);
                        token.set_type(class_scopes.unwrap()[0])
                    }
                }
            }
        }
    }
}

pub fn create_ownership(typed_tokens: Vec<TypedToken>, mode: CompilationMode) -> Vec<Node<TypedToken>> {
    let mut node_tokens: Vec<Node<TypedToken>> = vec![];
    let mut parents: Vec<Node<TypedToken>> = vec![];


    for typed_token in typed_tokens {
        if mode == CompilationMode::StubFile {
            node_tokens.append_item(Node::new(typed_token));
        } else if typed_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::ParenthesisOpen) {
            if parents.is_empty() {
                parents.push(Node::new(typed_token));
            } else {
                let new_parent = Node::new(typed_token);
                //parents.get(parents.len() - 1).expect("PARENT MUST EXIST").add_child_and_update_its_parent(&new_parent);
                parents.push(new_parent);
            }

            //parent_vec_pointer = TTRefTrack::new(std::ptr::addr_of_mut!(parent_vec_pointer), Box::new(*typed_token.clone().children));
        } else if typed_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::ParenthesisClosed) {
            if !parents.is_empty() {
                let p = parents.pop().unwrap();
                if !parents.is_empty() {
                    parents[parents.len()-1].add_child_and_update_its_parent(&p);
                } else {
                    node_tokens.append_item(p);
                }
                node_tokens.append_item(Node::new(typed_token));
            }
        } else if !parents.is_empty() {
            parents.last().expect("PARENT MUST EXIST").create_and_add_child(typed_token);
        } else {
            node_tokens.append_item(Node::new(typed_token));
        }
    }
    if !parents.is_empty() {
        node_tokens.push(parents.pop().unwrap())
    }
    node_tokens
}
