use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use filepath::FilePath;
use crate::{BOUNDARY, CompileVocab, Fabric, TypedToken};
use crate::CompileVocab::{CLASS, CONSTANT, CONSTRUCTOR, FUNCTION, KEYWORD, MODULE, OPERATOR, SPECIAL, TYPE, UNKNOWN, VARIABLE};
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::compiler::lang::leblanc_keywords::keyword_value;
use crate::leblanc::compiler::lang::leblanc_lang::{boundary_value, function_type_value, special_value};
use crate::leblanc::compiler::lang::leblanc_operators::operator_type;
use crate::leblanc::compiler::symbols::{Symbol, SymbolType};
use crate::leblanc::core::native_types::type_value;
use crate::leblanc::rustblanc::Appendable;
use crate::leblanc::rustblanc::relationship::Node;
use crate::LeBlancType::Class;

pub fn create_stub_dump(fabric: &mut Fabric) {
    let mut output = fabric.imports().iter().map(|i| i.source.clone()).collect::<Vec<String>>().join("|") + "\n";

    let file = File::create(fabric.path.replace(".lb", ".lbsf"));
    println!("File: {:?}", file);
    fabric.tokens().iter().for_each(|t| output += &(t.value.as_stub_string() + "\n"));
    file.unwrap().write_all(output.as_bytes()).unwrap();
}

pub fn read_from_stub_dump(file: File) -> Fabric {
    let path = file.path().unwrap().to_str().unwrap().to_string();
    let file_reader = BufReader::new(file);
    let mut tokens = vec![];

    let mut lines = file_reader.lines();
    let _imports = lines.next().unwrap().unwrap();
    //let imports = imports.split("|").map(|s| s.to_string()).collect::<Vec<String>>();


    for line in lines {
        tokens.append_item(Node::new(parse_stub_token(line.unwrap())));
    }

    return Fabric::new(path, tokens, vec![], vec![], vec![]);
}

pub fn parse_stub_token(line: String) -> TypedToken {
    let line_number_sep = line.find("|").unwrap();
    let line_number = line[0..line_number_sep].to_string().parse::<u32>().unwrap();
    let line = line[line_number_sep+1..].to_string();

    let symbol_length_sep = line.find("|").unwrap();
    let symbol_length = line[0..symbol_length_sep].to_string().parse::<u32>().unwrap();
    let line = line[symbol_length_sep+1..].to_string();

    let symbols = line[0..symbol_length as usize].to_string();
    let symbol_number_sep = symbols.find("|").unwrap();
    let mut symbol_number = symbols[0..symbol_number_sep].to_string().parse::<u32>().unwrap() - 1;
    let symbols = symbols[symbol_number_sep+1..].to_string();
    let mut symbol_vec = vec![];
     symbols.chars().for_each(|c| {
        symbol_number += 1;
        symbol_vec.append_item(Symbol::new(c, false, false, false, false, SymbolType::Unknown, symbol_number, line_number));
    });
    let token = Token::new(symbol_vec, line_number);

    let line = line[symbol_length as usize..].to_string();

    let vocab_sep = line.find("|").unwrap();
    let vocab = line[0..vocab_sep].to_string();
    let line = line[vocab_sep+1..].to_string();
    let vocab_type = match_leblanc_type(vocab);


    let scope_sep = line.find("|").unwrap();
    let scope = line[0..scope_sep].to_string().parse::<i32>().unwrap();

    let global_sep = line.find("|").unwrap();
    let global_value = line[0..global_sep].to_string().parse::<i32>().unwrap();
    let global = global_value == 1;

    let class_member_sep = line.find("|").unwrap();
    let class_member_value = line[0..class_member_sep].to_string().parse::<i32>().unwrap();
    let class_member = class_member_value == 1;

    let mut typed_token = TypedToken::new(token, vocab_type, scope, global, class_member);

    let mut line = line[scope_sep+1..].to_string();
    let mut typings = vec![];
    while line[0..2].to_string() != "&&" {
        let vocab_sep = line.find("|").unwrap();
        let vocab = line[0..vocab_sep].to_string();
        line = line[vocab_sep+1..].to_string();
        let vocab_type = type_value(&vocab);
        typings.append_item(vocab_type);
    }
    if !typings.is_empty() {
        typed_token.set_typing_returns(typings);
    }


    return typed_token;

}

fn match_leblanc_type(vocab_string: String) -> CompileVocab {
    let vocab_string_sep = vocab_string.find(".");
    let vocab_string_sep = vocab_string_sep.unwrap();
    let first_vocab = vocab_string[0..vocab_string_sep].to_string();
    let second_vocab = vocab_string[vocab_string_sep+1..].to_string();
    return match first_vocab.as_str() {
        "constant" => CONSTANT(type_value(&second_vocab)),
        "variable" => VARIABLE(type_value(&second_vocab)),
        "constructor" => CONSTRUCTOR(type_value(&second_vocab)),
        "class" => CLASS(type_value(&second_vocab)),
        "type" => TYPE(type_value(&second_vocab)),
        "unknown" => UNKNOWN(type_value(&second_vocab)),
        "operator" => OPERATOR(operator_type(&second_vocab)),
        "special" => SPECIAL(special_value(&second_vocab), 120),
        "keyword" => KEYWORD(keyword_value(&second_vocab)),
        "module" => MODULE(second_vocab.parse::<u64>().unwrap()),
        "boundary" => BOUNDARY(boundary_value(&second_vocab.chars().next().unwrap())),
        "function" => FUNCTION(function_type_value(&second_vocab)),
        _ => UNKNOWN(Class(0))
    };

}