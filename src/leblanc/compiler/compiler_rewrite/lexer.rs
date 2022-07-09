pub mod tokens;

use std::fs;
use crate::pest::Parser;
use std::fs::File;
use std::io::Read;


#[derive(Parser)]
#[grammar = "grammar/leblanc.pest"] // relative to src
pub struct LeblancParser;

pub fn lex(string: String) {
    let unparsed_file = fs::read_to_string("test.lb").expect("cannot read file");

    println!("Unparsed: {:#?}", unparsed_file);
    let file = LeblancParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(; // get and unwrap the `file` rule; never fails


    println!("{:#?}", file);
}
