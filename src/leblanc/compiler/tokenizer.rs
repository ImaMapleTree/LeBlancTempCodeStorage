use std::borrow::Borrow;
use crate::CompilationMode;
use crate::leblanc::compiler::char_reader::CharReader;
use crate::leblanc::compiler::compiler_util::{CharMarker};
use crate::leblanc::compiler::identifier::token::Token;
use crate::leblanc::compiler::identifier::token_typer::create_typed_tokens;
use crate::leblanc::compiler::lang::leblanc_constants::constant_type;
use crate::leblanc::compiler::lang::leblanc_lang::is_special;
use crate::leblanc::compiler::lang::leblanc_operators::{is_operator, LBOperator, operator_type};
use crate::leblanc::compiler::partial::PartialFabric;
use crate::leblanc::compiler::symbols::{Symbol, SymbolType};
use crate::leblanc::compiler::symbols::SymbolType::Whitespace;
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::exception::leblanc_base_exception::LeblancBaseException;
use crate::leblanc::rustblanc::lib::leblanc_colored::{Color, colorize, ColorString};


pub fn create_tokens(char_reader: &mut CharReader, mode: CompilationMode) -> PartialFabric {
    let mut partial_errors: Vec<ErrorStub> = Vec::new();


    let mut quote_marker = CharMarker::new('\0', 0, 0);

    let mut tokens: Vec<Token> = Vec::new();


    let c = char_reader.advance(1);
    let c_future = char_reader.char_next();


    let mut next_symbol = Symbol::new(c, c_future.is_whitespace(),
                                      false, check_start_quote(c, quote_marker.ch), false,
                                      SymbolType::of(c), char_reader.symbol_number(), char_reader.line_number());


    let mut token: Token = Token::empty();

    while !char_reader.eof() {
        let current_symbol = next_symbol;
        next_symbol = get_next_symbol(char_reader, current_symbol, &mut quote_marker);

        //println!("{:#?}", token);

        // If our current token starts with a quote indicator then we keep adding to the token until we get the end quote indicator
        if !token.last_symbol_or_empty().is_end_quote && token.first_symbol_or_empty().is_start_quote {
            token.add_symbol(current_symbol)
        } else if token.last_symbol_or_empty().is_end_quote {
            add_token(&mut tokens, token);
            token = Token::from(current_symbol);
        }
        else if token.as_string().starts_with("//") {
            token.add_symbol(current_symbol);
            if *next_symbol.char() == '\n' {
                add_token(&mut tokens, token);
                token = Token::empty();
            }
        }
        else if token.as_string().starts_with("/*") {
            token.add_symbol(current_symbol);
            if token.as_string().ends_with("*/") {
                add_token(&mut tokens, token);
                token = Token::empty();
            }
        }
        // Here we check if the current symbol + next symbol is an operator so that we can with things like "=="
        else if is_operator((current_symbol.as_string() + &next_symbol.as_string()).as_str()) {
            add_token(&mut tokens, token);
            token = Token::from(current_symbol);
        }
        else if is_special((current_symbol.as_string() + &next_symbol.as_string()).as_str()) {
            add_token(&mut tokens, token);
            token = Token::from(current_symbol);
            let csymbol = next_symbol;
            next_symbol = get_next_symbol(char_reader, current_symbol, &mut quote_marker);
            token.add_symbol(csymbol);
            if !(token.as_string().starts_with("//") || token.as_string().starts_with("/*")) {
                add_token(&mut tokens, token);
                token = Token::empty();
            }
        }
        // Check if the symbol is a character that counts as an individual token or operator
        else if current_symbol.is_boundary() || is_operator(current_symbol.as_string().as_str()) {
            // If the last token is only an operator then we can add the new operator to it
            if operator_type(current_symbol.as_string().as_str()) == LBOperator::Attribute && constant_type(token.as_string().as_str()).is_numeric() {
                token.add_symbol(current_symbol);
            }
            else if is_operator(current_symbol.as_string().as_str()) && is_operator(token.as_string().as_str()) {
                token.add_symbol(current_symbol);
                add_token(&mut tokens, token);
                token = Token::empty();
            } else {
                add_token(&mut tokens, token);
                token = Token::from(current_symbol);
                add_token(&mut tokens, token);
                token = Token::empty();
            }
        }
        else if *current_symbol.get_type() == Whitespace {
            add_token(&mut tokens, token);
            token = Token::empty();
        }
        else {
            token.add_symbol(current_symbol);
        }
    }


    /*
     * We don't throw an error here because we want to give a better error report to the user
     * So we're going to continue with code execution until we get more errors then we give a better report of what happened
     * and where we were unbalanced
     */
    if quote_marker.ch != '\0' {
        partial_errors.push(ErrorStub::ParseImbalancedQuotation(quote_marker.line_number, quote_marker.symbol_number));
        //LeblancBaseException::new(&(error_details + &ColorString::new(&format!("\nParse Error at ({}::{}) -> Unclosed Quotation", quote_marker.line_number, quote_marker.symbol_number)).red().bold().string())
        //                         , true, 5009002).throw();
    }


    tokens.reverse();
    let mut fabric = create_typed_tokens(tokens, partial_errors, mode);
    fabric.path = char_reader.path().clone();
    return fabric;

}

fn get_next_symbol(char_reader: &mut CharReader, last_symbol: Symbol, mut quote_marker: &mut CharMarker) -> Symbol {
    let c = char_reader.advance(1);
    let cfuture = char_reader.char_next();

    /*
     * Here we check for the borders of the quote
     */
    let quote_start = check_start_quote(c, quote_marker.ch);
    let mut quote_end = false;
    if quote_start {
        quote_marker.ch = c;
        quote_marker.symbol_number = char_reader.symbol_number();
        quote_marker.line_number = char_reader.line_number();
    } else {
        quote_end = check_end_quote(c, quote_marker.ch);
        if quote_end {
            quote_marker.ch = '\0';
            quote_marker.symbol_number = char_reader.symbol_number();
            quote_marker.line_number = char_reader.line_number();
        }
    }


    return Symbol::new(c, cfuture.is_whitespace(),
                       last_symbol.get_type() == &SymbolType::Whitespace,
                       quote_start, quote_end,
                       SymbolType::of(c), char_reader.symbol_number(), char_reader.line_number());
}

fn check_start_quote(ch: char, quote_marker: char) -> bool {
    match ch {
        '"' => quote_marker == '\0' && quote_marker != ch,
        '\'' => quote_marker == '\0' && quote_marker != ch,
        _ => false
    }
}

fn check_end_quote(ch: char, quote_marker: char) -> bool {
    match ch {
        '"' => quote_marker == ch,
        '\'' => quote_marker == ch,
        _ => false
    }
}

fn add_token(tokens: &mut Vec<Token>, token: Token) {
    //println!("Adding token: {}", token.as_string());
    if !(token.as_string().starts_with("//") || token.as_string().starts_with("/*")) {
        if token.borrow().len() > 0 {
            tokens.push(token);
        }
    }
}


fn print_string(string: &String) {
    println!("{}", string);
}