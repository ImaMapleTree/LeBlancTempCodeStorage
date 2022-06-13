use std::collections::HashMap;
use crate::{BraceOpen, CompilationMode, CompileVocab, LeBlancType, TypedToken};
use crate::CompileVocab::FUNCTION;
use crate::leblanc::compiler::identifier::typed_token::PartialToken;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword::{Func, Returns};
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType;
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{BraceClosed, Comma};
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
        if token.lang_type() == FUNCTION {
            func_matcher.insert(token.as_partial(), token.typing().clone());
        }
    }


    return create_ownership(typed_tokens, func_matcher, mode);
}

pub fn identify_functions(typed_tokens: &mut Vec<TypedToken>, errors: &mut Vec<ErrorStub>) -> HashMap<PartialToken, Vec<LeBlancType>>{
    let mut func_matcher: HashMap<PartialToken, Vec<LeBlancType>> = HashMap::new();
    let mut ndi = 0;
    let mut returns = false;
    let mut return_types = vec![];
    for i in 0..typed_tokens.len() {
        let token = &typed_tokens[i];
        if token.lang_type() == CompileVocab::KEYWORD(Func) {
            if i+1 > typed_tokens.len() {
                println!("My error was here");
                errors.insert(errors.len(), InvalidSyntax(token.clone()));
            } else {
                if ndi != 0 {
                    func_matcher.insert(typed_tokens[ndi].as_partial(), vec![Class(0)]);
                    typed_tokens[ndi].set_typing(vec![Class(0)]);
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
                typed_tokens[ndi].set_typing(return_types);
                return_types = vec![];
                ndi = 0;
            } else if let CompileVocab::TYPE(return_type) = token.lang_type() {
                return_types.insert(return_types.len(), return_type);
            } else if token.lang_type() != CompileVocab::BOUNDARY(Comma) {
                errors.insert(errors.len(), InvalidSyntax(token.clone()));
            }
        }
    }
    return func_matcher;
}

pub fn identify_unknown(typed_tokens: &mut Vec<TypedToken>, type_map: &mut HashMap<String, Vec<Vec<CompileVocab>>>) {
    for mut token in typed_tokens.iter_mut().filter(|t| t.lang_type().matches("unknown")) {
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

pub fn create_ownership(typed_tokens: Vec<TypedToken>, func_matcher: HashMap<PartialToken, Vec<LeBlancType>>, mode: CompilationMode) -> Vec<Node<TypedToken>> {
    let mut node_tokens: Vec<Node<TypedToken>> = vec![];
    let mut parents: Vec<Node<TypedToken>> = vec![];

    for mut typed_token in typed_tokens {
        let partial_token = typed_token.as_partial();
        if func_matcher.contains_key(&partial_token) {
            typed_token.set_typing(func_matcher.get(&partial_token).unwrap().clone());
        }
        if mode == CompilationMode::StubFile {
            node_tokens.insert(node_tokens.len(), Node::new(typed_token));
        } else {
            if typed_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::ParenthesisOpen) {
                if parents.is_empty() {
                    parents.push(Node::new(typed_token));
                } else {
                    let new_parent = Node::new(typed_token);
                    parents.get(parents.len() - 1).expect("PARENT MUST EXIST").add_child_and_update_its_parent(&new_parent);
                    parents.push(new_parent);
                }

                //parent_vec_pointer = TTRefTrack::new(std::ptr::addr_of_mut!(parent_vec_pointer), Box::new(*typed_token.clone().children));
            } else if typed_token.lang_type() == CompileVocab::BOUNDARY(BoundaryType::ParenthesisClosed) {
                if !parents.is_empty() {
                    node_tokens.insert(node_tokens.len(), parents.pop().unwrap());
                }
            } else {
                if !parents.is_empty() {
                    parents.get(parents.len() - 1).expect("PARENT MUST EXIST").create_and_add_child(typed_token);
                } else {
                    node_tokens.insert(node_tokens.len(), Node::new(typed_token));
                }
            }
        }
    }
    return node_tokens;
}

