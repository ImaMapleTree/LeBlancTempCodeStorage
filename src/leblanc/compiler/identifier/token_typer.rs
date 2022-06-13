use std::any::Any;
use std::borrow::{BorrowMut, Cow};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use regex::internal::Input;
use crate::leblanc::compiler::compiler_util::string_is_in_array;
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::compiler::identifier::token_identifier::identify;
use crate::leblanc::compiler::lang::leblanc_constants::{constant_type, is_constant};
use crate::leblanc::compiler::lang::leblanc_keywords::{is_keyword, keyword_value, LBKeyword};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword::{Extension, Func, Of, Using};
use crate::leblanc::compiler::lang::leblanc_lang::{boundary_value, is_special, special_value, CompileVocab, BoundaryType};
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab::{CLASS, CONSTANT, EXTENSION, FUNCTION, KEYWORD, MODULE, TYPE, UNKNOWN, VARIABLE};
use crate::leblanc::compiler::lang::leblanc_operators::{is_operator, LBOperator, operator_type};
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator::{Not, Plus};
use crate::leblanc::compiler::partial::PartialFabric;
use crate::leblanc::compiler::symbols::Symbol;
use crate::leblanc::compiler::syntax_rules::RuleAnalyzer;
use crate::leblanc::compiler::identifier::token_typer::GlobalScopeMarker::*;
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::core::native_types::LeBlancType::{Class, Flex};
use crate::leblanc::core::native_types::{is_native_type, type_value};
use crate::leblanc::core::native_types::class_type::ClassMeta;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub::FlexReassignment;
use crate::leblanc::rustblanc::relationship::Node;
use crate::{CompilationMode, compile, LeBlancType};



pub fn create_typed_tokens<'a>(mut tokens: Vec<Token>, mut errors: Vec<ErrorStub>, mode: CompilationMode) -> PartialFabric {
    let UNKNOWN_TYPE: LeBlancType = Class(0);
    let UNKNOWN_VOCAB: CompileVocab = UNKNOWN(UNKNOWN_TYPE);

    let mut imports: Vec<String> = Vec::new();

    let mut typed_tokens: Vec<TypedToken> = Vec::new();
    let mut type_map: HashMap<String, Vec<Vec<CompileVocab>>> = HashMap::new();

    let mut new_class_counter = 0;
    let mut brace_counter = 0;
    let mut scope_value = 0;
    let mut class_scope: u32 = 0;
    let mut global_scope = NotGlobal;
    let mut lock_global_scope = false;

    let mut token = Token::empty();
    let mut next_token = tokens.pop().unwrap_or(Token::empty());

    let follower_types = &["func".to_string(), "Extension".to_string(), "Class".to_string()];

    //Code Analysis
    let mut analysis = RuleAnalyzer::new();


    while tokens.len() > 0 || next_token != Token::empty() {
        token = next_token;
        next_token = tokens.pop().unwrap_or(Token::empty());


        let token_string = token.as_string();
        let first_symbol = token.first_symbol_or_empty();

        match first_symbol.char() {
            '(' => analysis.add_parenthesis(first_symbol, typed_tokens[typed_tokens.len()-1].lang_type().clone()),
            ')' => analysis.add_parenthesis(first_symbol, typed_tokens[typed_tokens.len()-1].lang_type().clone()),
            _ => {},
        }

        let type_scopes = type_map.get(token_string.as_str());

        let vocab =
            if type_scopes.is_some() && !is_keyword(token.as_string().as_str()) && !first_symbol.is_boundary() && type_scopes.unwrap().len()  > scope_value {
                let type_scopes = type_scopes.unwrap();
                if *next_token.first_symbol_or_empty().char() == '(' {
                    if let TYPE(temp_type) = type_scopes[scope_value][0] {
                        CompileVocab::CONSTRUCTOR(temp_type.clone())
                    } else {
                        FUNCTION
                    }
                } else {
                    type_scopes[scope_value][0]
                }
            }

            else if is_special(token_string.as_str()) {
                CompileVocab::SPECIAL(special_value(token_string.as_str()))
            } else if is_operator(token_string.as_str()) {
                CompileVocab::OPERATOR(operator_type(token_string.as_str()))
            } else if is_keyword(token_string.as_str()) {
                let keyword = keyword_value(token_string.as_str());
                if keyword == Func {
                    scope_value += 1;
                    global_scope = FuncDeclaration;
                } else if keyword == LBKeyword::Class {
                    scope_value += 1;
                    global_scope = ClassDeclaration;
                }
                KEYWORD(keyword)
            } else if is_constant(token_string.as_str()) {
                CONSTANT(constant_type(token_string.as_str()))
            } else if first_symbol.is_boundary() {
                match first_symbol.char() {
                    '{' => brace_counter += 1,
                    '}' => {
                        brace_counter -= 1;
                        if brace_counter == 0 {
                            class_scope = 0;
                        }
                    },
                    ';' => analysis.evaluate_rule1(&mut errors),
                    _ => {}
                }
                CompileVocab::BOUNDARY(boundary_value(first_symbol.char()))
            } else if typed_tokens.len() > 0 && typed_tokens[typed_tokens.len() - 1].as_string().as_str() == "using" {
                let index = imports.iter().position(|r| r == &token_string);
                let index = match index {
                    None => {
                        let mut import_name = token_string.clone();
                        if !import_name.ends_with(".lb") {
                            import_name += ".lb";
                        }
                        imports.insert(imports.len(), import_name);
                        imports.len() - 1
                    }
                    Some(position) => position
                };
                MODULE(index as u64)
            } else if typed_tokens.len() > 0 && string_is_in_array(&typed_tokens[typed_tokens.len() - 1].as_string(), follower_types) {
                let last_token_type = typed_tokens[typed_tokens.len() - 1].lang_type();
                if let KEYWORD(value) = last_token_type {
                    match value {
                        Func => {
                            FUNCTION
                        },
                        Extension => EXTENSION(UNKNOWN_TYPE),
                        LBKeyword::Class => {
                            new_class_counter += 1;
                            CLASS(Class(0))
                        }
                        _ => UNKNOWN_VOCAB
                    }
                } else {
                    UNKNOWN_VOCAB
                }

            } else {
                if is_native_type(token_string.as_str()) {
                    let vocab_type = TYPE(type_value(token_string.as_str()));
                    vocab_type
                } else {
                    if let TYPE(inner_type) = typed_tokens.get((typed_tokens.len() as i32 - 1) as usize).unwrap_or(&TypedToken::empty()).lang_type() {
                        VARIABLE(inner_type)
                    } else {
                        let optional_scope = type_map.get(token_string.as_str());
                        if optional_scope.is_some() &&
                                !optional_scope.unwrap().get(scope_value).unwrap_or(&vec![]).is_empty() {
                            let scopes = optional_scope.unwrap();
                            let block_value = scopes.get(scope_value).unwrap();
                            block_value[0]
                        } else {
                            if next_token.first_symbol_or_empty().is_boundary() {
                                if *next_token.first_symbol_or_empty().char() == '(' {
                                    FUNCTION
                                } else {
                                    UNKNOWN(Class(0))
                                }
                            } else {
                                if is_constant(next_token.as_string().as_str()) {
                                    UNKNOWN_VOCAB
                                } else {
                                    new_class_counter += 1;
                                    TYPE(Class(0))
                                }
                            }
                        }
                    }
                }
            };


        if vocab == KEYWORD(LBKeyword::Global) || vocab.matches("class") {
            global_scope = GlobalBlock;
        }

        println!("Token: {} | State: {:?}", token, global_scope);
        let temp_scope_value = scope_value;

        if global_scope != NotGlobal || lock_global_scope {
            scope_value = 0;
        }

        if type_map.contains_key(token_string.as_str()) {
            let mut nested_vec: &mut Vec<Vec<CompileVocab>> = type_map.get_mut(token_string.as_str()).unwrap();
            if nested_vec.len() > scope_value {
                let mut type_vec: &mut Vec<CompileVocab> = nested_vec.get_mut(scope_value).unwrap();
                type_vec.insert(type_vec.len(), vocab);
            } else {
                for i in 0..scope_value - nested_vec.len() {
                    nested_vec.insert(nested_vec.len(), Vec::new());
                }
                let mut type_vec = Vec::new();
                type_vec.push(vocab);
                nested_vec.insert(nested_vec.len(), type_vec);
            }
        } else {
            let mut nested_vec: Vec<Vec<CompileVocab>> = Vec::new();
            for i in 0..scope_value {
                nested_vec.push(Vec::new());
            }
            let mut type_vec = Vec::new();
            type_vec.push(vocab);
            nested_vec.insert(nested_vec.len(), type_vec);
            type_map.insert(token_string, nested_vec);
        }

        /*if vocab.matches("type") {
            if exists_in_scope(&type_map, next_token.as_string(), scope_value as i32) {
                errors.insert(errors.len(), ErrorStub::VariableAlreadyDefined(TypedToken::new(next_token.copy(), vocab.clone(), scope_value as i32)))
            }
        }*/

        let mut typed_token = TypedToken::new(token, vocab, scope_value as i32, global_scope != NotGlobal);

        scope_value = temp_scope_value;

        if scope_value == 0 || brace_counter == 0 {
            if !(global_scope != NotGlobal || lock_global_scope) && typed_tokens.len() > 0 {
                match vocab {
                    KEYWORD(_) => {}
                    MODULE(_) => {}
                    CompileVocab::BOUNDARY(_) => {}
                    EXTENSION(_) => {}
                    CLASS(_) => {
                        if typed_tokens[typed_tokens.len() - 1].lang_type() != CompileVocab::KEYWORD(LBKeyword::Class) {
                            errors.insert(errors.len(), ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                        }
                    }
                    TYPE(_) => {
                        if typed_tokens[typed_tokens.len() - 1].lang_type() != CompileVocab::KEYWORD(Of) {
                            errors.insert(errors.len(), ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                        }
                    }
                    FUNCTION => {
                        println!("LAST TYPE!!: {:?}", typed_tokens[typed_tokens.len() - 1].lang_type());
                        if typed_tokens[typed_tokens.len() - 1].lang_type() != CompileVocab::KEYWORD(Func) {
                            errors.insert(errors.len(), ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                        }
                    }
                    _ => errors.insert(errors.len(), ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                }
            }
        }


        global_scope = if  allow_global_scope_end(global_scope, *typed_token.token().first_symbol_or_empty().char()) { NotGlobal } else { global_scope };

        if vocab.matches("class") {
            global_scope = NotGlobal;
            class_scope = type_map.get("Class").unwrap().len() as u32;
            println!("CLASS SCOPE: {}", class_scope);
        }
        
        typed_tokens.insert(typed_tokens.len(), typed_token);
    }

    let mut import_tokens = vec![];
    println!("Creating imports!");
    imports.iter().for_each(|import| import_tokens.append(compile(import.clone(), CompilationMode::StubFile).tokens()));
    println!("Creating imports!");

    let mut node_tokens = identify(typed_tokens, import_tokens, &mut type_map, &mut errors, mode);
    println!("Creating imports!");

    analysis.evaluate(&mut errors, &mut node_tokens);

    return PartialFabric::no_path(node_tokens, imports, errors);
}

fn exists_in_scope(type_map: &HashMap<String, Vec<Vec<CompileVocab>>>, value: String, scope_value: i32) -> bool {
    let scopes = type_map.get(&value);
    if scopes.is_none() {
        return false;
    }
    let scopes = scopes.unwrap();
    let scope_container = scopes.get(scope_value as usize);
    if scope_container.is_none() {
        return false;
    }
    let scope_container = scope_container.unwrap();
    return !scope_container.is_empty();
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum GlobalScopeMarker {
    FuncDeclaration,
    ClassDeclaration,
    GlobalBlock,
    GlobalLine,
    NotGlobal
}

fn allow_global_scope_end(scope: GlobalScopeMarker, ch: char) -> bool {
    return match scope {
        FuncDeclaration => ch == '{',
        ClassDeclaration => ch == '}',
        GlobalBlock => ch == '}',
        GlobalLine => ch == ';',
        NotGlobal => ch == '\0',
    }
}