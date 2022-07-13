use std::fs::File;
use std::io::{BufReader, Read};


use crate::leblanc::compiler::bytecode::LeblancBytecode;


use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::utils::encode_hex;

pub fn read_file(path: String) -> LeblancBytecode {
    let file = File::open(path.replace(".lb", ".lbbc")).unwrap();
    let file_reader = BufReader::new(file);
    let hex = encode_hex(&file_reader.bytes().map(|l| l.unwrap()).collect::<Vec<u8>>());

    

    LeblancBytecode::from(hex)
}

pub fn read_bytecode(hex: Hexadecimal) -> LeblancBytecode {
    LeblancBytecode::from(hex)
}