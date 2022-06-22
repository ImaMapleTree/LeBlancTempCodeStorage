use crate::leblanc::core::native_types::LeBlancType;

pub fn is_constant(string: &str) -> bool { return constant_type(string) != LeBlancType::Class(0) }

pub fn constant_type(string: &str) -> LeBlancType {
    if string == "true" { return LeBlancType::Boolean; }
    if string == "false" { return LeBlancType::Boolean; }
    if string.starts_with("\"") && string.ends_with("\"") { return LeBlancType::String; }
    if string.starts_with("'") && string.ends_with("'") { return LeBlancType::Char; }
    let mut number_string = string;
    let mut last_char = '\0';
    if string.len() > 1 && !string.chars().last().unwrap().is_numeric() {
        last_char = string.chars().last().unwrap();
        number_string = &number_string[0..number_string.len() - 1];
    }
    if number_string.chars().all(|c| c.is_numeric() || c == '.') {
        match last_char.to_ascii_uppercase() {
            'L' => return LeBlancType::Int64,
            'F' => return LeBlancType::Float,
            'D' => return LeBlancType::Double,
            _ => {}
        }

        if string.contains(".") {
            if string.len() - string.find(".").unwrap() > 7 {
                return LeBlancType::Double;
            } else {
                return LeBlancType::Float;
            }
        }
        if string.len() > 10 {
            return LeBlancType::Int64;
        }
        return LeBlancType::Int
    }
    return LeBlancType::Class(0);
}