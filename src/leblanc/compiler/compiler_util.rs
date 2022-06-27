use crate::leblanc::rustblanc::relationship::Node;
use crate::{to_node_vec, TypedToken};
use crate::leblanc::rustblanc::AppendCloneable;

pub struct CharMarker {
    pub(crate) ch: char,
    pub(crate) symbol_number: u32,
    pub(crate) line_number: u32
}

impl CharMarker {
    pub fn new(ch: char, symbol_number: u32, line_number: u32) -> CharMarker {
        CharMarker {
            ch, symbol_number, line_number
        }
    }
}

pub fn count_min_leading_whitespace(lines: &Vec<String>) -> u32 {
    let mut min_check = false;
    let mut min_whitespace = 0;
    for line in lines {
        let mut string_temp = line.as_str();
        let mut whitespace = 0;
        while string_temp.starts_with('\t') || string_temp.starts_with(' ') {
            string_temp = &string_temp[1..];
            whitespace += 1;
        }
        if !min_check {
            min_whitespace = whitespace;
            min_check = true;
        } else if min_whitespace > whitespace {
            min_whitespace = whitespace;
        }

    }
    min_whitespace
}

pub fn line_strip_and_join(lines: &Vec<String>) -> String {
    let mut joined = String::new();
    let strip_count = count_min_leading_whitespace(lines) as usize;
    for line in lines {
        joined += &("\n".to_owned() + &line[strip_count..]);
    }
    joined = joined[1..].parse().unwrap();
    joined
}

pub fn strip_start_of_line(mut string: String) -> String {
    let mut string_temp = string.as_str();
    while !string.is_empty() && string.starts_with('\t') || string.starts_with(' ') {
        string_temp = &string[1..];
        string = string_temp.to_string();
    }
    string
}

pub fn string_is_in_array(string: &String, array: &[String]) -> bool {
    array.contains(string)
}

pub fn flatmap_node_tokens(tokens: &mut Vec<Node<TypedToken>>) -> Vec<TypedToken> {
    let mut flatmap = vec![];
    for token in tokens {
        flatmap.append_clone(&token.value);
        if !token.children.read().unwrap().is_empty() {
            flatmap.append(&mut flatmap_node_tokens(&mut to_node_vec(&token.children)));
        }

    }
    flatmap
}