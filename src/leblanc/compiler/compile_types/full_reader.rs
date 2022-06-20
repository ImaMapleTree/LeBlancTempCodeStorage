use std::fs::File;
use std::io::{BufRead, BufReader, Bytes, Read};
use crate::CompilationMode;
use crate::leblanc::core::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::core::bytecode::LeblancBytecode;
use crate::leblanc::core::interpreter::instructions::Instruction;
use crate::leblanc::core::interpreter::run;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::utils::encode_hex;

pub fn read_file(path: String) -> LeblancBytecode {
    let file = File::open(path.replace(".lb", ".lbbc")).unwrap();
    let file_reader = BufReader::new(file);
    let hex = encode_hex(&file_reader.bytes().map(|l| l.unwrap()).collect::<Vec<u8>>());

    let bc = LeblancBytecode::from(hex);

    return bc;
}

pub fn read_bytecode(hex: Hexadecimal) -> LeblancBytecode {
    LeblancBytecode::from(hex)
}