use std::collections::HashMap;
use fxhash::hash;
use crate::{BraceOpen, CompilationMode, CompileVocab, LeBlancType, Semicolon, TypedToken};
use crate::CompileVocab::{CONSTANT, FUNCTION, TYPE, UNKNOWN, VARIABLE};
use crate::leblanc::compiler::compile_types::partial_token::PartialToken;
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::compiler::identifier::token_identifier::LambdaCloser::{BraceCloser, SemiColonCloser};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword::{Func, Return, Returns, SelfRT};
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, FunctionType};
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::{BraceClosed, BracketClosed, BracketOpen, Comma, ParenthesisClosed, ParenthesisOpen};
use crate::leblanc::compiler::lang::leblanc_lang::Specials::LambdaMarker;
use crate::leblanc::compiler::symbols::Symbol;
use crate::leblanc::core::internal::methods::builtins::create_partial_functions;
use crate::leblanc::rustblanc::Appendable;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub::InvalidSyntax;
use crate::leblanc::rustblanc::relationship::Node;
use crate::LeBlancType::{Class, Flex};

pub fn identify(mut typed_tokens: Vec<TypedToken>, mut import_tokens: Vec<Node<TypedToken>>, type_map: &mut HashMap<String, Vec<Vec<CompileVocab>>>, mut func_matcher: HashMap<PartialToken, Vec<Vec<LeBlancType>>>, errors: &mut Vec<ErrorStub>, mode: CompilationMode) -> Vec<Node<TypedToken>>{

    identify_unknown(&mut typed_tokens, type_map);
    identify_functions(&mut typed_tokens, &mut func_matcher, errors);
    import_tokens.reverse();
    while !import_tokens.is_empty() {
        let token = &import_tokens.pop().unwrap().value;
        if token.lang_type() == FUNCTION(FunctionType::Header) {
            func_matcher.insert(token.as_partial(), vec![token.typing()[0].clone(), token.typing()[1].clone()]);
        }
    }

    for token in &mut typed_tokens {
        let mut partial_token = token.as_partial();
        let mut function_ref = false;
        if partial_token.lang_type == UNKNOWN(Class(0)) {
            partial_token.lang_type = FUNCTION(FunctionType::Call);
            function_ref = true;
        }
        if func_matcher.contains_key(&partial_token) {
            token.set_typing_returns(func_matcher.get(&partial_token).unwrap()[1].clone());
            if function_ref { token.set_type(FUNCTION(FunctionType::Reference))}
        }
    }
    create_anno_functions(&mut typed_tokens, errors);

    let typed_tokens = identify_caller_function_args2(&mut typed_tokens, None);


    for token in &typed_tokens {
        println!("Before owwnership: {:?}", token);
    }

    create_ownership(typed_tokens, mode)
}

pub fn identify_caller_function_args2(typed_tokens: &mut Vec<TypedToken>, mut func_call: Option<TypedToken>) -> Vec<TypedToken> {
    let mut new_typed_tokens = vec![];

    while !typed_tokens.is_empty() {
        let mut was_token = false;
        let mut token = match func_call {
            Some(token) => {
                was_token = true;
                func_call = None;
                token
            },
            None => typed_tokens.remove(0)
        };
        if token.lang_type() == FUNCTION(FunctionType::Call) || token.lang_type() == FUNCTION(FunctionType::ReferenceCall) {
            if token.lang_type() == FUNCTION(FunctionType::ReferenceCall) {
                token.set_typing_returns(vec![Flex])
            }
            let mut func_tokens = vec![];
            let mut func_param = TypedToken::empty();
            let mut can_add_type = true;
            let mut hit_boundary = false;
            let mut parenthesis_amount = 0;
            while !hit_boundary {
                func_param = typed_tokens.remove(0);
                if func_param.lang_type() == FUNCTION(FunctionType::Call) || func_param.lang_type() == FUNCTION(FunctionType::ReferenceCall) {
                    let mut token_list = identify_caller_function_args2(typed_tokens, Some(func_param));
                    if can_add_type { token.set_typing_args(&mut token_list[0].typing()[1].clone()); }
                    func_tokens.append(&mut token_list);
                    can_add_type = false;
                } else {
                    match func_param.lang_type() {
                        CONSTANT(value) | VARIABLE(value) | TYPE(value) => {
                            if can_add_type { token.set_typing_args(&mut vec![value])}
                            can_add_type = false;
                        }
                        FUNCTION(func_type) => {
                            if can_add_type && func_type == FunctionType::Reference { token.set_typing_args(&mut vec![LeBlancType::Function]); can_add_type = false;}
                        }
                        CompileVocab::BOUNDARY(boundary) => {
                            match boundary {
                                ParenthesisOpen => parenthesis_amount += 1,
                                ParenthesisClosed => {
                                    parenthesis_amount -= 1;
                                    if parenthesis_amount == 0 { hit_boundary = true }
                                }
                                Semicolon => hit_boundary = true,
                                Comma => can_add_type = true,
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    func_tokens.push(func_param);
                }
            }
            new_typed_tokens.push(token);
            new_typed_tokens.append(&mut func_tokens);
            if was_token { return new_typed_tokens }


        } else {
            new_typed_tokens.push(token);
        }

    }
    new_typed_tokens

}

pub fn identify_caller_function_args(typed_tokens: &mut Vec<TypedToken>, left_bound: usize, right_bound: usize) -> usize {
    let length = typed_tokens.len();
    let mut i_addition = 0;
    for mut i in left_bound..right_bound {
        i += i_addition;
        if typed_tokens.len() > i && (typed_tokens[i].lang_type() == FUNCTION(FunctionType::Call) || typed_tokens[i].lang_type() == FUNCTION(FunctionType::ReferenceCall)) && length > i+1 {
            let mut can_add_type = true;
            let mut j_addition = 0;
            for mut j in i+1..length {
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

pub fn identify_functions(typed_tokens: &mut Vec<TypedToken>, func_matcher: &mut HashMap<PartialToken, Vec<Vec<LeBlancType>>>, errors: &mut Vec<ErrorStub>) {

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
                    func_matcher.insert(typed_tokens[ndi].as_partial(), vec![vec![Class(0)], vec![]]);
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
                func_matcher.insert(typed_tokens[ndi].as_partial(), vec![arg_types.clone(), return_types.clone()]);
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
}

pub fn create_anno_functions(typed_tokens: &mut Vec<TypedToken>, errors: &mut Vec<ErrorStub>) {
    let mut i = 0;
    let mut final_appends = vec![];
    while i < typed_tokens.len() {
        let token = &typed_tokens[i];
        let mut lambda_args = vec![];
        let mut line_number = 0;
        if token.lang_type() == CompileVocab::SPECIAL(LambdaMarker, 120) {
            line_number = token.token().line_number();
            i += 1;
            let mut arg_token = typed_tokens.remove(i);
            let mut can_add = true;
            while arg_token.lang_type() != CompileVocab::SPECIAL(LambdaMarker, 120) {
                if arg_token.lang_type() == CompileVocab::BOUNDARY(Comma) {
                    if !can_add { can_add = true; } else { /*Big bad error here */ }
                } else {
                    lambda_args.push(arg_token);
                    can_add = false;
                }
                arg_token = typed_tokens.remove(i);
            }
            let mut instructs = vec![];
            let mut next_token = typed_tokens.remove(i);
            let mut closer = SemiColonCloser(1);
            if next_token.lang_type() == CompileVocab::BOUNDARY(BraceOpen) { closer = BraceCloser(0); }
            if next_token.lang_type() == CompileVocab::BOUNDARY(ParenthesisOpen) { closer = SemiColonCloser(0); }
            while !closer.should_break(next_token.as_string()) {
                instructs.push(next_token);
                next_token = typed_tokens.remove(i);
                if next_token.lang_type() == CompileVocab::BOUNDARY(BraceClosed) {
                    if let BraceCloser(n) = closer { closer = BraceCloser(n-1)}
                } else if next_token.lang_type() == CompileVocab::BOUNDARY(BraceOpen) {
                    if let BraceCloser(n) = closer { closer = BraceCloser(n+1)}
                } else if next_token.lang_type() == CompileVocab::BOUNDARY(ParenthesisClosed) {
                    if let SemiColonCloser(n) = closer { closer = SemiColonCloser(n-1)}
                } else if next_token.lang_type() == CompileVocab::BOUNDARY(ParenthesisOpen) {
                    if let SemiColonCloser(n) = closer { closer = SemiColonCloser(n+1)}
                }
            }
            let mut semicolon = Token::from_string(";".to_string());
            semicolon.set_line_number(line_number);
            let semicolon_token = TypedToken::new(semicolon, CompileVocab::BOUNDARY(Semicolon), arg_token.scope(), arg_token.global(), arg_token.class_member());
            typed_tokens.insert(i, next_token.clone());
            if closer == SemiColonCloser(0) {
                next_token = semicolon_token.clone();
            }
            instructs.push(next_token);
            /*            let return_token = TypedToken::new(Token::from_string("return".to_string()), CompileVocab::KEYWORD(Return), arg_token.scope(), arg_token.global(), arg_token.class_member());
                        instructs.push(return_token);*/
            let func_token = TypedToken::new(Token::from_string("func".to_string()), CompileVocab::KEYWORD(Func), 0, true, false);
            let name = hash(&instructs).to_string();
            let mut name_ref_token = Token::from_string(name.clone());
            name_ref_token.set_line_number(line_number);
            let mut semicolon = Token::from_string(";".to_string());
            semicolon.set_line_number(line_number);
            let func_name_token = TypedToken::new(Token::from_string(name), CompileVocab::FUNCTION(FunctionType::Header), 0, true, false);
            let parenthesis_open = TypedToken::new(Token::from_string("(".to_string()), CompileVocab::BOUNDARY(ParenthesisOpen), 0, true, false);
            let func_ref_token = TypedToken::new(name_ref_token, FUNCTION(FunctionType::Reference), arg_token.scope(), arg_token.global(), arg_token.class_member());
            let flex_token = TypedToken::new(Token::from_string("flex".to_string()), CompileVocab::TYPE(LeBlancType::Flex), 0, true, false);
            let comma_token = TypedToken::new(Token::from_string(",".to_string()), CompileVocab::BOUNDARY(Comma), 0, true, false);
            typed_tokens.insert(i, func_ref_token);
            final_appends.push(func_token);
            final_appends.push(func_name_token);
            final_appends.push(parenthesis_open);
            let mut add_comma = false;
            for arg in lambda_args {
                if add_comma { final_appends.push(comma_token.clone()) }
                final_appends.push(flex_token.clone());
                final_appends.push(arg);
                add_comma = true;

            }
            let parenthesis_closed = TypedToken::new(Token::from_string(")".to_string()), CompileVocab::BOUNDARY(ParenthesisClosed), 0, true, false);
            final_appends.push(parenthesis_closed);
            let returns_token = TypedToken::new(Token::from_string("returns".to_string()), CompileVocab::KEYWORD(Returns), 0, true, false);
            final_appends.push(returns_token);
            final_appends.push(flex_token);
            let brace_open_token = TypedToken::new(Token::from_string("{".to_string()), CompileVocab::BOUNDARY(BraceOpen), 0, true, false);
            let brace_closed_token = TypedToken::new(Token::from_string("}".to_string()), CompileVocab::BOUNDARY(BraceClosed), 0, false, false);
            final_appends.push(brace_open_token);
            final_appends.append(&mut instructs);
            final_appends.push(brace_closed_token);
        }
        i += 1;
    }
    /*for token in &final_appends {
        println!("FINAL APPEND TOKEN: {:?}", token);
    }*/
    typed_tokens.append(&mut final_appends);
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
    let mut parent_type = BoundaryType::DNE;

    for typed_token in typed_tokens {
        let lang_type = typed_token.lang_type();
        if mode == CompilationMode::StubFile {
            node_tokens.append_item(Node::new(typed_token));
        }

        else if lang_type == CompileVocab::BOUNDARY(ParenthesisOpen) || lang_type == CompileVocab::BOUNDARY(BracketOpen) {
            if let CompileVocab::BOUNDARY(boundary_type) = typed_token.lang_type() { parent_type = boundary_type }
            if parents.is_empty() {
                parents.push(Node::new(typed_token));
            } else {
                let new_parent = Node::new(typed_token);
                //parents.get(parents.len() - 1).expect("PARENT MUST EXIST").add_child_and_update_its_parent(&new_parent);
                parents.push(new_parent);
            }

            //parent_vec_pointer = TTRefTrack::new(std::ptr::addr_of_mut!(parent_vec_pointer), Box::new(*typed_token.clone().children));
        } else if (parent_type == ParenthesisOpen && lang_type == CompileVocab::BOUNDARY(ParenthesisClosed)) || (parent_type == BracketOpen && lang_type == CompileVocab::BOUNDARY(BracketClosed)) {
            parent_type = BoundaryType::DNE;
            if !parents.is_empty() {
                let p = parents.pop().unwrap();
                if !parents.is_empty() {
                    parents[parents.len()-1].add_child_and_update_its_parent(&p);
                    parents[parents.len()-1].add_child_and_update_its_parent(&Node::new(typed_token));
                    if let CompileVocab::BOUNDARY(boundary_type) = parents.last().unwrap().value.lang_type() { parent_type = boundary_type }
                } else {
                    node_tokens.append_item(p);
                    node_tokens.append_item(Node::new(typed_token));
                }
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

#[derive(Copy, Clone, PartialEq)]
enum LambdaCloser {
    BraceCloser(i64),
    SemiColonCloser(i64)
}

impl LambdaCloser {
    pub fn should_break(&self, other: String) -> bool {
        match self {
            BraceCloser(n) => {
                if *n == 0 {
                    other == "{"
                } else { false }

            },
            SemiColonCloser(n) => {
                if *n == 0 {
                    other == ")"
                } else {
                    other == ";"
                }
            }
        }
    }
}