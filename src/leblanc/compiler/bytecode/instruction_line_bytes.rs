use crate::leblanc::compiler::bytecode::byte_limiter::ByteLimit::Limited;
use crate::leblanc::compiler::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::compiler::bytecode::ToBytecode;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

#[derive(Debug, Clone)]
pub struct InstructionBytecode {
    line_number: ByteRestriction,
    instructions: ByteRestriction,
    instruction_arguments: ByteRestriction
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
            instruction_arguments: ByteRestriction::repeated(Limited(2))
        }
    }

    pub fn set_line_number(&mut self, line_number: u32) {
        self.line_number.consume_bytes(line_number.to_hex(128)).expect("Line number too many bytes");
    }

    pub fn add_instruction(&mut self, instruction: Hexadecimal, instruction_argument: Hexadecimal) {
        self.instructions.consume_bytes(instruction).expect("instruction too many bytes");
        self.instruction_arguments.consume_bytes(instruction_argument).expect("instruction arg too many bytes");
    }

    pub fn from(hex: &mut Hexadecimal) -> InstructionBytecode {
        let mut bytecode = InstructionBytecode::new();
        let line_number = hex.scrape(bytecode.line_number.unpack().unwrap() as usize);

        while !hex.is_empty() {
            let instruction = hex.scrape(bytecode.instructions.unpack().unwrap() as usize);
            let instruction_arg = hex.scrape(bytecode.instruction_arguments.unpack().unwrap() as usize);
            bytecode.instructions.consume_bytes(instruction).unwrap();
            bytecode.instruction_arguments.consume_bytes(instruction_arg).unwrap();
        }

        bytecode.line_number.consume_bytes(line_number).unwrap();

        bytecode
    }

    pub fn is_empty(&mut self) -> bool {
        self.instructions.segments().unwrap().is_empty()
    }

    pub fn to_instructions(mut self) -> Vec<Instruction> {
        let mut instructions = self.instructions.segments().unwrap();
        let mut instruction_args = self.instruction_arguments.segments().unwrap();
        let line_number = self.line_number.bytes().to_hexable::<u32>();
        let mut mapped = vec![];
        while !instructions.is_empty() {
            mapped.push(Instruction::new(InstructionBase::from_hex(&instructions.remove(0)), instruction_args.remove(0).to_hexable::<u16>(), line_number));
        }
        mapped
    }

    pub fn remove(&mut self) -> (Hexadecimal, Hexadecimal) {
        let instruction = self.instructions.pop().unwrap();
        let arg = self.instruction_arguments.pop().unwrap();
        (instruction, arg)
    }
}

impl ToBytecode for InstructionBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let instruction_bytes = self.instructions.join(&self.instruction_arguments);

        self.line_number.bytes() + instruction_bytes
    }
}