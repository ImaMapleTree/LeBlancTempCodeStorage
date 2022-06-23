use std::collections::HashMap;
use crate::leblanc::compiler::compiler_util::string_is_in_array;
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::compiler::identifier::token_identifier::identify;
use crate::leblanc::compiler::lang::leblanc_constants::{constant_type, is_constant};
use crate::leblanc::compiler::lang::leblanc_keywords::{is_keyword, keyword_value, LBKeyword};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword::{ExtensionImport, Func, Of};
use crate::leblanc::compiler::lang::leblanc_lang::{boundary_value, is_special, special_value, CompileVocab, FunctionType, Specials};
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab::{CLASS, CONSTANT, EXTENSION, FUNCTION, KEYWORD, MODULE, TYPE, UNKNOWN, VARIABLE};
use crate::leblanc::compiler::lang::leblanc_operators::{is_operator, operator_type};
use crate::leblanc::compiler::fabric::Fabric;
use crate::leblanc::compiler::syntax_rules::RuleAnalyzer;
use crate::leblanc::compiler::identifier::token_typer::GlobalScopeMarker::*;
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::core::native_types::LeBlancType::Class;
use crate::leblanc::core::native_types::{is_native_type, type_value};
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::{CompilationMode, compile, LeBlancType};
use crate::leblanc::compiler::import::{Import, ImportType};
use crate::leblanc::compiler::lang::leblanc_lang::ExtensionType::{ExtensionTypeParam, ExtensionTypeExport, ExtensionTypeImport};
use crate::leblanc::rustblanc::Appendable;


pub fn create_typed_tokens<'a>(mut tokens: Vec<Token>, mut errors: Vec<ErrorStub>, mode: CompilationMode) -> Fabric {
    let UNKNOWN_TYPE: LeBlancType = Class(0);
    let UNKNOWN_VOCAB: CompileVocab = UNKNOWN(UNKNOWN_TYPE);

    let mut imports: Vec<Import> = Vec::new();
    let mut extensions: Vec<String> = Vec::new();

    let mut typed_tokens: Vec<TypedToken> = Vec::new();
    let mut type_map: HashMap<String, Vec<Vec<CompileVocab>>> = HashMap::new();

    let mut new_class_counter = 0;
    let mut brace_counter = 0;
    let mut scope_value = 0;
    let mut class_scope: u32 = 0;
    let mut global_scope = NotGlobal;
    let lock_global_scope = false;

    let mut token = Token::empty();
    let mut next_token = tokens.pop().unwrap_or_else(Token::empty);

    let follower_types = &["func".to_string(), "Extension".to_string(), "Class".to_string(), "ext".to_string(),
        "from".to_string(), "of".to_string(), "using".to_string()];

    //Code Analysis
    let mut analysis = RuleAnalyzer::new();


    while tokens.len() > 0 || next_token != Token::empty() {
        token = next_token;
        next_token = tokens.pop().unwrap_or_else(Token::empty);


        let token_string = token.as_string();
        let first_symbol = token.first_symbol_or_empty();

        match first_symbol.char() {
            '(' => analysis.add_parenthesis(first_symbol, typed_tokens[typed_tokens.len()-1].lang_type().clone()),
            ')' => analysis.add_parenthesis(first_symbol, typed_tokens[typed_tokens.len()-1].lang_type().clone()),
            _ => {},
        }

        let type_scopes = type_map.get(token_string.as_str());

        let mut vocab =
            if type_scopes.is_some() && !is_keyword(token.as_string().as_str()) && !first_symbol.is_boundary() && type_scopes.unwrap().len()  > scope_value {
                let type_scopes = type_scopes.unwrap();
                if *next_token.first_symbol_or_empty().char() == '(' {
                    if let TYPE(temp_type) = type_scopes[scope_value][0] {
                        CompileVocab::CONSTRUCTOR(temp_type.clone())
                    } else {
                        FUNCTION(FunctionType::Call)
                    }
                } else {
                    type_scopes[scope_value][0]
                }
            }

            else if is_special(token_string.as_str()) {
                match token_string.as_str() {
                    "->" => CompileVocab::SPECIAL(special_value(token_string.as_str()), 0),
                    _ => CompileVocab::SPECIAL(special_value(token_string.as_str()), 120)
                }
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
            } else if typed_tokens.len() > 0 && typed_tokens[typed_tokens.len() - 1].lang_type() == KEYWORD(ExtensionImport){
                let import = Import::new(&token_string, &token_string, ImportType::Extension);
                let index = match imports.iter().cloned().position(|i| i == import) {
                    Some(position) => position,
                    None => {
                        imports.push(import);
                        imports.len() - 1
                    }
                };
                EXTENSION(ExtensionTypeImport(index as u32))
            } else if typed_tokens.len() > 0 && string_is_in_array(&typed_tokens[typed_tokens.len() - 1].as_string(), follower_types) {
                let last_token_type = typed_tokens[typed_tokens.len() - 1].lang_type();
                if let KEYWORD(value) = last_token_type {
                    match value {
                        LBKeyword::Using => {
                            if !(token_string == "ext" || token_string == "extension") {
                                let import = Import::new(&token_string, &token_string, ImportType::File);
                                let index = match imports.iter().cloned().position(|i| i == import) {
                                    Some(position) => position,
                                    None => {
                                        imports.push(import);
                                        imports.len() - 1
                                    }
                                };
                                MODULE(index as u64)
                            } else {
                                EXTENSION(ExtensionTypeImport(0))
                            }
                        }
                        Func => {
                            FUNCTION(FunctionType::Header)
                        }
                        LBKeyword::From => {
                            let incorrect_import = imports.pop().unwrap();
                            println!("Incorrect import: {:#?}", incorrect_import);
                            let import_type = if incorrect_import.import_type == ImportType::Extension { ImportType::Extension } else { ImportType::SubImport };

                            let import = Import::new(&incorrect_import.name, &token_string, import_type);
                            let index = match imports.iter().cloned().position(|i| i == import) {
                                Some(position) => position,
                                None => {
                                    imports.push(import);
                                    imports.len() - 1
                                }
                            };
                            if import_type == ImportType::Extension {
                                EXTENSION(ExtensionTypeImport(index as u32))
                            } else {
                                MODULE(index as u64)
                            }
                        }
                        LBKeyword::Extension => {
                            global_scope = GlobalScopeMarker::ExtensionDeclaration;
                            let index = if extensions.contains(&token_string) {
                                extensions.iter().position(|ext| ext == &token_string).unwrap()
                            } else {
                                extensions.push(token_string.clone());
                                extensions.len() - 1
                            };
                            EXTENSION(ExtensionTypeExport(index as u32))
                        }
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
                if token_string == "class" {
                    TYPE(Class(0))
                } else if is_native_type(token_string.as_str()) {
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
                                    FUNCTION(FunctionType::Call)
                                } else {
                                    UNKNOWN(Class(0))
                                }
                            } else {
                                if is_constant(next_token.as_string().as_str()) {
                                    UNKNOWN_VOCAB
                                } else {
                                    if next_token.as_string() == "=" || next_token.as_string() == "->" {
                                        UNKNOWN(Class(0))
                                    } else {
                                        new_class_counter += 1;
                                        TYPE(Class(0))
                                    }
                                }
                            }
                        }
                    }
                }
            };


        if global_scope == GlobalScopeMarker::ExtensionDeclaration && vocab == UNKNOWN_VOCAB {
            vocab = EXTENSION(ExtensionTypeParam(type_value(&token_string)))
        }

        if vocab == KEYWORD(LBKeyword::Global) || vocab.matches("class") {
            global_scope = GlobalBlock;
        }

        let temp_scope_value = scope_value;

        if global_scope == GlobalLine || global_scope == GlobalBlock || lock_global_scope {
            scope_value = 0;
        }

        if type_map.contains_key(token_string.as_str()) {
            let nested_vec: &mut Vec<Vec<CompileVocab>> = type_map.get_mut(token_string.as_str()).unwrap();
            if nested_vec.len() > scope_value {
                let type_vec: &mut Vec<CompileVocab> = nested_vec.get_mut(scope_value).unwrap();
                type_vec.append_item( vocab);
            } else {
                for _i in 0..scope_value - nested_vec.len() {
                    nested_vec.append_item( Vec::new());
                }
                let mut type_vec = vec![vocab];
                nested_vec.append_item(type_vec);
            }
        } else {
            let mut nested_vec: Vec<Vec<CompileVocab>> = Vec::new();
            for _i in 0..scope_value {
                nested_vec.push(Vec::new());
            }
            let mut type_vec = Vec::new();
            if let EXTENSION(extension_type) = vocab {
                if let ExtensionTypeParam(_type_param)  = extension_type {
                    // This code is really gross
                } else {
                    type_vec.push(vocab);
                    nested_vec.append_item( type_vec);
                    type_map.insert(token_string, nested_vec);
                }

            } else {
                if !vocab.matches("special") {
                    type_vec.push(vocab);
                    nested_vec.append_item(type_vec);
                    type_map.insert(token_string, nested_vec);
                }
            }

        }

        let class_member = typed_tokens.len() > 0 && typed_tokens[typed_tokens.len() - 1].lang_type() == CompileVocab::SPECIAL(Specials::Dot, 120);

        if vocab.matches("type") {
            if exists_in_scope(&type_map, next_token.as_string(), scope_value as i32) && !next_token.first_symbol_or_empty().is_boundary() {
                errors.push(ErrorStub::VariableAlreadyDefined(TypedToken::new(next_token.copy(), vocab.clone(), scope_value as i32, global_scope != NotGlobal, class_member)))
            }
        }

        let typed_token = TypedToken::new(token, vocab, scope_value as i32, global_scope != NotGlobal, class_member);

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
                            errors.append_item(ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                        }
                    }
                    TYPE(_) => {
                        if typed_tokens[typed_tokens.len() - 1].lang_type() != CompileVocab::KEYWORD(Of) {
                            errors.append_item( ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                        }
                    }
                    FUNCTION(_) => {
                        println!("LAST TYPE!!: {:?}", typed_tokens[typed_tokens.len() - 1].lang_type());
                        if typed_tokens[typed_tokens.len() - 1].lang_type() != CompileVocab::KEYWORD(Func) {
                            errors.append_item(ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                        }
                    }
                    _ => errors.append_item(ErrorStub::InvalidGlobalVariableDeclaration(typed_token.clone()))
                }
            }
        }


        global_scope = if  allow_global_scope_end(global_scope, *typed_token.token().first_symbol_or_empty().char()) { NotGlobal } else { global_scope };

        if vocab.matches("class") {
            global_scope = NotGlobal;
            class_scope = type_map.get("Class").unwrap().len() as u32;
        }

        //println!("typed token: {:?}", typed_token);

        typed_tokens.append_item(typed_token);
    }

    let mut import_tokens = vec![];
    imports.iter_mut().for_each(|import| {
        if !import.source.contains('.') {
            import.source = import.source.clone() + ".lb";
        }
        import_tokens.append(compile(import.source.clone(), CompilationMode::StubFile).tokens())
    });

    let mut node_tokens = identify(typed_tokens, import_tokens, &mut type_map, &mut errors, mode);


    analysis.evaluate(&mut errors, &mut node_tokens);

    return Fabric::no_path(node_tokens, imports, vec![], errors);
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
    ExtensionDeclaration,
    GlobalBlock,
    GlobalLine,
    NotGlobal
}

fn allow_global_scope_end(scope: GlobalScopeMarker, ch: char) -> bool {
    return match scope {
        FuncDeclaration => ch == '{',
        ClassDeclaration => ch == '}',
        ExtensionDeclaration => ch == '{',
        GlobalBlock => ch == '}',
        GlobalLine => ch == ';',
        NotGlobal => ch == '\0',
    }
}