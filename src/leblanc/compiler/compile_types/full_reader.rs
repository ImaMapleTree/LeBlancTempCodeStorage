use std::fs::File;
use std::io::{BufRead, BufReader, Bytes, Read};
use crate::CompilationMode;
use crate::leblanc::core::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::core::bytecode::LeblancBytecode;
use crate::leblanc::core::interpreter::frame::FrameBase;
use crate::leblanc::core::interpreter::instructions::Instruction;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::utils::encode_hex;

pub fn read_file(path: String) -> LeblancBytecode {
    let mut file = File::open(path.replace(".lb", ".lbbc")).unwrap();
    let file_reader = BufReader::new(file);
    let mut hex = encode_hex(&file_reader.bytes().map(|l| l.unwrap()).collect::<Vec<u8>>());

    let mut bc = LeblancBytecode::from(hex);
    for mut function in bc.body().functions() {
        let mut instructs: Vec<Instruction> = vec![];
        function.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));
        println!("instructs: {:#?}", instructs);
    }
    return bc;

}

pub fn read_bytecode(hex: Hexadecimal) -> LeblancBytecode {
    LeblancBytecode::from(hex)
}