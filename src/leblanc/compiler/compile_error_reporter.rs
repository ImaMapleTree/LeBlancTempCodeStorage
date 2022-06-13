use std::process::exit;
use regex::Regex;
use crate::leblanc::compiler::char_reader::CharReader;
use crate::leblanc::compiler::compiler_util::{CharMarker, line_strip_and_join, strip_start_of_line};
use crate::leblanc::compiler::lang::leblanc_lang::CompileVocab::{CONSTRUCTOR, FUNCTION};
use crate::leblanc::compiler::identifier::typed_token::TypedToken;
use crate::leblanc::rustblanc::exception::error_stubbing::ErrorStub;
use crate::leblanc::rustblanc::exception::leblanc_base_exception::LeblancBaseException;
use crate::leblanc::rustblanc::lib::leblanc_colored::{Color, ColorBright, colorize, colorize_str, ColorString};
use crate::leblanc::rustblanc::lib::leblanc_colored::Color::Bright;
use crate::leblanc::rustblanc::lib::leblanc_colored::ColorBright::{BrightGreen, BrightWhite};


fn create_dashes_to_symbol(number: u32, symbol_number: u32, string: String) -> String {
    let mut s = String::new();
    let real_number = if number > 2 {
        number - 2
    } else {
        0
    };
    for i in 0..real_number {
        s += "-";
    }

    //println!("Symbol number: {}", symbol_number);
    //println!("String: {}", string);
    //println!("String: {}", string.len());

    if (symbol_number as i32) - (string.len() as i32) > 0 {
        s += " ";
    }

    s += &generate_arrows(string);
    //return colorize_str(s.as_str(), Color::Bright(ColorBright::BrightYellow));
    return ColorString::new(s.as_str()).colorize(Color::Bright(ColorBright::BrightYellow)).bold().string()
}


pub fn error_report(cr: &mut CharReader, tokens: &Vec<TypedToken>, errors: &Vec<ErrorStub>) {
    let digits = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut complete_errors: Vec<LeblancBaseException> = Vec::new();
    let error = errors[0].clone();
    let line_number = match error {
        ErrorStub::ParseImbalancedQuotation(line_number, _) => { line_number }
        ErrorStub::ImbalancedDelimiter(Symbol) => { Symbol.line_number() }
        ErrorStub::MissingSemicolon(line_number, _) => { line_number }
        ErrorStub::UndeclaredVariable(ref typed) => { typed.token().line_number() }
        ErrorStub::InvalidGlobalVariableDeclaration(ref typed) => {typed.token().line_number()}
        ErrorStub::FlexReassignment(ref typed) => {typed.token().line_number()}
        ErrorStub::IncompatibleType(ref typed) => {typed.token().line_number()}
        ErrorStub::VariableAlreadyDefined(ref typed) => {typed.token().line_number()}
        ErrorStub::InvalidSyntax(ref typed) => {typed.token().line_number()}
    };
    let mut symbol_number = 0;

    let line_tokens = get_line_tokens(&tokens, line_number);

    let mut exact_token = TypedToken::empty();
    let mut fix = String::new();

    let error_line: String = cr.lines()[(line_number as usize)-1].clone();
    let stripped_error_line = strip_start_of_line(error_line.clone());
    let strip_amount = error_line.len() - stripped_error_line.len();
    let mut error_name = String::new();
    let mut error_message_extra = String::new();
    let mut error_syntax = String::new();

    if let ErrorStub::UndeclaredVariable(ref undeclared) = error {
        error_name = String::from("Undeclared Variable");
        symbol_number = undeclared.token().first_symbol_or_empty().symbol_number();
        error_syntax = undeclared.as_string();

        for token in tokens.iter().filter(|t| t.scope() == 0 || t.scope() == undeclared.scope()) {
            if (token.as_string().starts_with(&undeclared.as_string()) || undeclared.as_string().starts_with(&token.as_string()))
            && !token.as_string().eq(&undeclared.as_string()) {
                fix = bright_white(format!("\nNote - There's a similar variable named: {} in this scope",
                            colorize(token.as_string(), Color::Green)).as_str());
                break;
            }
        }
        error_message_extra += &("\n".to_owned() + &fix);
    } else if let ErrorStub::InvalidGlobalVariableDeclaration(ref undeclared) = error {
        error_name = String::from("Global declaration of variable without 'global' keyword");
        symbol_number = undeclared.token().first_symbol_or_empty().symbol_number();
        error_syntax = error_line.clone();
        fix = bright_white("\nPossible Fix - ") + &ColorString::new("Add 'global' keyword to declaration").green().string();
        error_message_extra += &("\n".to_owned() + &fix
            + "\n| " + &line_number.to_string() + " \t" + repair_missing_global_error(error_line).as_str());
    } else if let ErrorStub::IncompatibleType(ref incompatible) = error {
        error_name = String::from("Attempt to assign variable an incompatible type.");
        let line_tokens = get_line_tokens(&tokens, line_number);
        symbol_number = line_tokens[0].token().first_symbol_or_empty().symbol_number();
        error_syntax = stripped_error_line.clone();
        let mut declaration_line = Option::None;
        for token in tokens.iter().filter(|t| t.scope() == 0 || t.scope() == incompatible.scope()) {
            if token.as_string() == incompatible.as_string() {
                declaration_line = Option::Some(token.token().line_number());
                break;
            }
        }
        let assignee_var_type = if declaration_line.is_some() && declaration_line.unwrap() != line_number {
            String::from("dynamic")
        } else {
            line_tokens[3].lang_type().extract_native_type().as_str().to_string()
        };

        fix = bright_white("\nPossible Fix - ") + &ColorString::new(&format!("Change assignee variable type to '{}'", assignee_var_type)).green().string();

        if declaration_line.is_some() {
            let declaration_line = declaration_line.unwrap();
            error_message_extra += &("\n".to_owned() + &fix
                + "\n| " + &declaration_line.to_string() + " \t"
                + repair_incompatible_type_error(cr.lines()[(declaration_line as usize)-1].clone(), incompatible, assignee_var_type).as_str());
        }
    }
    else if let ErrorStub::VariableAlreadyDefined(imbalanced) = error {
        error_name = String::from("Variable already declared in this scope");
        println!("tokens: {:?}", &tokens);
        symbol_number = get_line_tokens(&tokens, line_number)[0].token().first_symbol_or_empty().symbol_number();
        error_syntax = stripped_error_line.clone();
        fix = bright_white("\nPossible Fix - ") + &ColorString::new(&format!("Rename the new variable")).green().string();

        let similar_tokens: Vec<TypedToken> = tokens.iter().filter(|t|
            (t.scope() == imbalanced.scope()) && t.as_string().starts_with(&imbalanced.as_string().trim_end_matches(digits).to_owned())).map(|t| t.clone()).collect();
        let mut var_count = 1;
        let mut new_var_name = imbalanced.as_string().trim_end_matches(digits).to_owned() + &var_count.to_string();
        println!("Similar tokens: {:?}", similar_tokens);
        while similar_tokens.iter().any(|t| t.as_string() == new_var_name) {
            var_count += 1;
            new_var_name = imbalanced.as_string().trim_end_matches(digits).to_owned() + &var_count.to_string();
        }

        error_message_extra += &("\n".to_owned() + &fix
            + "\n| " + &line_number.to_string() + " \t" + repair_var_already_defined_error(error_line, imbalanced.as_string(), new_var_name).as_str());
    }

    else if let ErrorStub::ImbalancedDelimiter(imbalanced) = error {
        error_name = String::from("Unbalanced Delimiter");
        symbol_number = imbalanced.symbol_number();
        error_syntax = imbalanced.as_string();
        for token in line_tokens {
            let imbalance_char = imbalanced.char();
            let first_symbol = token.token().first_symbol_or_empty();

            if *imbalance_char == '(' {
                if first_symbol.is_boundary() {
                    exact_token = token;
                    fix = bright_white("\nPossible Fix - ") + &ColorString::new("Add closing ')'").green().string();
                }
            }
        }

        if exact_token != TypedToken::empty() {
            let insert_char = match imbalanced.char() {
                '(' => ')',
                ')' => '(',
                _ => '\0'
            };

            error_message_extra += &("\n".to_owned() + &fix
                + "\n" + &line_number.to_string() + "\t" + repair_syntax_error(error_line, exact_token, insert_char).as_str());
        }
    } else {
        error_name = String::from("Unknown Error");
        symbol_number = 1;
        error_syntax = stripped_error_line.clone();
        error_message_extra = colorize("\nExtra Error Information:\n".to_owned() + &error.to_string(), Bright(ColorBright::BrightMagenta));
    }



    let mut error_message = generate_file_path(cr.path(), line_number, symbol_number) + "\n\n"
        + &colorize(stripped_error_line, Color::Red)
        + &("\n".to_owned()
        + create_dashes_to_symbol((symbol_number - (strip_amount as u32)), (symbol_number - 1 - (strip_amount as u32) + error_syntax.len() as u32),error_syntax).as_str())
        + "\n" + &generate_error_name(error_name.as_str(), line_number, symbol_number);

    if error_message_extra != String::new() {
        error_message += &error_message_extra;
    }

    complete_errors.insert(complete_errors.len(),
       LeblancBaseException::new(&error_message, true, 5009003));

    for error in complete_errors {
        error.output();
    }
    exit(0)
}

fn get_line_tokens(all_tokens: &Vec<TypedToken>, line_number: u32) -> Vec<TypedToken> {
    let mut line_tokens: Vec<TypedToken> = Vec::new();
    all_tokens.iter().filter(|t| t.token().line_number() == line_number).for_each(|t| line_tokens.insert(line_tokens.len(), t.clone()));
    return line_tokens;
}

fn generate_file_path(path: &String, line_number: u32, symbol_number: u32) -> String {
    ColorString::new("Error in: ").bold().red().string() + path + ":" + &line_number.to_string() + ":" + &symbol_number.to_string()
}

fn repair_syntax_error(text: String, token: TypedToken, insert_char: char) -> String {
    let stripped = strip_start_of_line(text.clone());
    let strip_amount = text.len() - stripped.len();
    let last_symbol_index = token.token().last_symbol_or_empty().symbol_number() as usize - 1 - strip_amount;
    return colorize(stripped[..last_symbol_index].to_string() + &colorize(String::from(insert_char), Color::Bright(BrightGreen)) + &stripped[last_symbol_index..], Color::White);
}

fn repair_missing_global_error(text: String) -> String {
    let stripped = strip_start_of_line(text.clone());
    return colorize(String::from("global "), Color::Bright(ColorBright::BrightGreen)) + &colorize(stripped, Color::Yellow);
}

fn repair_incompatible_type_error(text: String, token: &TypedToken, fix_type: String) -> String {
    let stripped = strip_start_of_line(text.clone());
    let re = Regex::new(&format!("{}{}{}", r"\b", token.as_string(), r"\b")).unwrap();
    let p = re.find(&stripped).unwrap().start();
    return colorize((fix_type + " "), Color::Bright(ColorBright::BrightGreen)) + &colorize_str(&stripped[p..], Color::Yellow);
}

fn repair_var_already_defined_error(text: String, old_name: String, new_name: String) -> String {
    let stripped = strip_start_of_line(text.clone()).replacen(old_name.as_str(), &colorize(new_name, Bright(BrightGreen)), 1);
    return colorize(stripped, Color::Yellow);
}

fn generate_error_name(text: &str, line_number: u32, symbol_number: u32) -> String {
    return colorize(format!("Parse Error at: ({}::{}) -> {}", line_number, symbol_number, text), Color::Red)
}

fn generate_arrows(token_string: String) -> String {
    let mut arrows = String::new();
    for i in 0..token_string.len() {
        arrows += "^";
    }
    return arrows;
}

fn bright_white(string: &str) -> String {
    return colorize_str(string, Color::Bright(BrightWhite));
}

fn in_same_scope(token: TypedToken, scope: i32) -> bool {
    if token.scope() == 0 || token.scope() == scope {
        return true;
    }
    return false;
}