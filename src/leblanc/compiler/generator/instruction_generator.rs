use std::mem::take;
use crate::leblanc::compiler::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::rustblanc::Hexable;

#[derive(Default, Debug)]
pub struct InstructionGenerator {
    instructions: Vec<InstructionBytecode>,
    open: Vec<usize>,
    current: InstructionBytecode,
    last_instruction: Instruction2
}

impl InstructionGenerator {
    pub fn open(&mut self) {
        self.open.push(0);
    }

    pub fn open_with_amount(&mut self, amount: usize) {
        self.open.push(amount)
    }

    pub fn close(&mut self) -> usize {
        self.open.pop().expect("Closed with no open counts")
    }

    pub fn close_or_zero(&mut self) -> usize {
        self.open.pop().unwrap_or(0)
    }


    fn check_line_number(&mut self, line: u32) {
        if self.current.line_number() != line {
            let current = take(&mut self.current);
            if current.line_number() != 0 {
                self.instructions.push(current);
            }
            self.current.set_line_number(line)
        }
    }

    pub fn add_instruct_components(&mut self, instruction: InstructionBase, arg: u16, line: u32) {
        self.check_line_number(line);
        self.current.add_instruction(instruction, arg.to_hex(2));
        self.increment_open_count();
    }


    pub fn add_instruction(&mut self, line: usize, instruction: Instruction2) {
        self._add_instruction(line, instruction);
        self.increment_open_count();
    }

    pub fn instructions(&self) -> &Vec<InstructionBytecode> {
        &self.instructions
    }

    pub fn take_instructions(&mut self) -> Vec<InstructionBytecode> {
        self.check_line_number(0);
        take(&mut self.instructions)
    }

    pub fn bytecode_lines(&self) -> usize {
        self.instructions.len()
    }

    pub fn get_bytecode_mut(&mut self, index: usize) -> Option<&mut InstructionBytecode> {
        self.instructions.get_mut(index)
    }

    pub fn last_instruction(&self) -> Instruction2 {
        self.last_instruction
    }

    pub fn refresh(&mut self) {
        let current = take(&mut self.current);
        let line = current.line_number();
        if line != 0 {
            self.instructions.push(current);
        }
        self.current.set_line_number(line)
    }

    pub fn remove_last(&mut self) -> Instruction2 {
        self.current.remove();
        for i in 0..self.open.len() {
            self.open[i] -= 1;
        }
        self.last_instruction
    }

    fn _add_instruction(&mut self, line: usize, instruction: Instruction2) {
        println!("{:?}", instruction);
        self.check_line_number(line as u32);
        self.current.add_instruction2(instruction);
        self.last_instruction = instruction;
    }

    fn increment_open_count(&mut self) {
        for i in 0..self.open.len() {
            self.open[i] += 1;
        }
    }


}