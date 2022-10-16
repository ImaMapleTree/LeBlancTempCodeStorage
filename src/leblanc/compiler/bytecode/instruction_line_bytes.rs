use crate::leblanc::compiler::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::compiler::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::compiler::bytecode::ToBytecode;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

#[derive(Debug, Clone)]
pub struct InstructionBytecode {
    line_number: ByteRestriction,
    instructions: ByteRestriction,
    instruction_info: ByteRestriction,
    line: u32
}

impl Default for InstructionBytecode {
    fn default() -> Self {
        InstructionBytecode::new()
    }
}

impl InstructionBytecode {
    pub fn new() -> InstructionBytecode {
        InstructionBytecode {
            line_number: ByteRestriction::once(Limited(4)),
            instructions: ByteRestriction::repeated(Limited(2)),
            instruction_info: ByteRestriction::repeated(Undefined),
            line: 0
        }
    }

    pub fn set_line_number(&mut self, line_number: u32) {
        self.line = line_number;
        self.line_number.consume_bytes(line_number.to_hex(128)).expect("Line number too many bytes");
    }

    pub fn line_number(&self) -> u32 {
        self.line
    }

    pub fn add_instruction(&mut self, instruction: InstructionBase, instruction_argument: Hexadecimal) {
        self.instructions.consume_bytes(instruction.to_hex(2)).expect("instruction too many bytes");
        self.instruction_info.consume_bytes(instruction_argument).expect("instruction arg too many bytes");
    }

    pub fn add_instruction2(&mut self, instruction: Instruction2) {
        let (ihex, ahex) = instruction.tuple_hex();
        self.instructions.consume_bytes(ihex).expect("instruction too many bytes");
        self.instruction_info.consume_bytes(ahex).expect("instruction args too many bytes.");
    }

    pub fn from(hex: &mut Hexadecimal) -> InstructionBytecode {
        let mut bytecode = InstructionBytecode::new();
        let line_number = hex.scrape(bytecode.line_number.unpack().unwrap() as usize);

        while !hex.is_empty() {
            let ihex = hex.scrape(bytecode.instructions.unpack().unwrap() as usize);
            let arg_count = Instruction2::supple_bytes(ihex.to_hexable::<u16>());
            let ahex = hex.scrape((arg_count * 2) as usize);
            bytecode.instructions.consume_bytes(ihex).unwrap();
            bytecode.instruction_info.consume_bytes(ahex).unwrap();
        }

        bytecode.line_number.consume_bytes(line_number).unwrap(); bytecode
    }

    pub fn is_empty(&mut self) -> bool {
        self.instructions.segments().unwrap().is_empty()
    }

    pub fn to_instructions(mut self) -> Vec<Instruction> {
        let mut instructions = self.instructions.segments().unwrap();
        let mut instruction_args = self.instruction_info.segments().unwrap();
        let line_number = self.line_number.bytes().to_hexable::<u32>();
        let mut mapped = vec![];
        while !instructions.is_empty() {
            mapped.push(Instruction::new(InstructionBase::from_hex(&instructions.remove(0)), instruction_args.remove(0).to_hexable::<u16>(), line_number));
        }
        mapped
    }

    pub fn to_instructions2(mut self) -> Vec<Instruction2> {
        let mut instructions = self.instructions.segments().unwrap();
        let mut instruction_args = self.instruction_info.segments().unwrap();
        let line_number = self.line_number.bytes().to_hexable::<u32>();
        let mut mapped = vec![];
        while !instructions.is_empty() {
            mapped.push(Instruction2::from((instructions.remove(0), instruction_args.remove(0), line_number)));
        } mapped
    }

    pub fn remove(&mut self) -> (Hexadecimal, Hexadecimal) {
        let instruction = self.instructions.pop().unwrap();
        let arg = self.instruction_info.pop().unwrap();
        (instruction, arg)
    }
}

impl ToBytecode for InstructionBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let instruction_bytes = self.instructions.join(&self.instruction_info);

        self.line_number.bytes() + instruction_bytes
    }
}