use std::borrow::Borrow;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use filepath::FilePath;
use crate::leblanc::rustblanc::exception::leblanc_base_exception::LeblancBaseException;

pub struct CharReader {
    file_path: String,
    lines: Vec<String>,
    characters: Vec<char>,
    line_number: u32,
    symbol_number: u32,
    char_history: Vec<char>,
    last_char: char,
    current_char: char,
    next_char: char,
    total_scanned_chars: u64,
    end_of_file: bool,
}

impl CharReader {
    pub fn new(file: File) -> CharReader {
        let mut lines: Vec<String> = Vec::new();
        let file_path = file.path().unwrap().to_str().unwrap().to_string();
        let file_reader = BufReader::new(file);
        let char_iterator = file_reader.lines().flat_map(|line_res| {
            let line = match line_res {
                Ok(c) => c,
                Err(error) => {
                    let b: Box<dyn Error> = error.into();
                    let borrowed: &Box<dyn Error> = b.borrow();
                    let string = borrowed.to_string();
                    LeblancBaseException::from(b, &format!("Error parsing lines in source file: {}", string)
                                               , true, 5006001).handle().unwrap()
                }
            };
            lines.push(line.clone());
            let mut chars = line.chars().collect::<Vec<_>>();
            chars.push('\n');
            return chars;
        });

        let characters: Vec<char> = char_iterator.collect();
        let next_char: char = **characters.get(0).get_or_insert(&'\0');


        return CharReader {
            file_path,
            lines,
            characters,
            line_number: 1,
            symbol_number: 0,
            char_history: Vec::new(),
            last_char: char::from(0),
            current_char: char::from(0),
            next_char,
            total_scanned_chars: 0,
            end_of_file: false
        }
    }

    pub fn from_line(line: String) -> CharReader {
        let lines: Vec<String> = line.lines().map(|line| line.to_string()).collect::<Vec<String>>();
        let file_path = "leblanc.exe".to_string();
        let characters: Vec<char> = line.chars().collect();
        let next_char: char = **characters.get(0).get_or_insert(&'\0');
        return CharReader {
            file_path,
            lines,
            characters,
            line_number: 1,
            symbol_number: 0,
            char_history: Vec::new(),
            last_char: char::from(0),
            current_char: char::from(0),
            next_char,
            total_scanned_chars: 0,
            end_of_file: false
        }
    }

    pub fn advance(&mut self, i: u32) -> char {
        for i in 0..i {
            self.last_char = self.current_char;
            self.current_char = self.next_char;
            self.next_char = **self.characters.get((self.total_scanned_chars+((i+1) as u64)) as usize).get_or_insert(&'\0');
            self.total_scanned_chars += 1 as u64;

            if self.next_char == '\0' {
                self.end_of_file = true;
            }

            self.symbol_number += 1;

            if self.current_char == '\n' {
                self.symbol_number = 0;
                self.line_number += 1;
            }


        }

        return self.current_char;
    }

    pub fn char_next(&self) -> &char {&self.next_char}

    pub fn lines(&self) -> &Vec<String> {&self.lines}

    pub fn line_number(&self) -> u32 {self.line_number}

    pub fn symbol_number(&self) -> u32 {self.symbol_number}

    pub fn path(&self) -> &String { &self.file_path }

    pub fn chars(&self) -> &Vec<char> {
        return &self.characters;
    }

    pub fn eof(&self) -> bool {
        return self.end_of_file;
    }

}