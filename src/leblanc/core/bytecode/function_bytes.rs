use crate::leblanc::core::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::core::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::core::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::core::bytecode::ToBytecode;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

pub struct FunctionBytecode {
    name_length: ByteRestriction,
    name: ByteRestriction,
    constants_total_length: ByteRestriction,
    constant_value_length: ByteRestriction,
    constant_value: ByteRestriction,
    constant_type: ByteRestriction,
    variable_total_length: ByteRestriction,
    variable_name_length: ByteRestriction,
    variable_name: ByteRestriction,
    variable_relationship: ByteRestriction,
    precompiled_total_length: ByteRestriction,
    precompiled_seg_length: ByteRestriction,
    precompiled_code: ByteRestriction,
    instructions_total_size: ByteRestriction,
    instruction_line_length: ByteRestriction,
    instruction_line: ByteRestriction,
}

impl FunctionBytecode {
    pub fn new() -> FunctionBytecode {
        return FunctionBytecode {
            name_length: ByteRestriction::once(Limited(4)),
            name: ByteRestriction::once(Undefined),
            constants_total_length: ByteRestriction::once(Limited(6)),
            constant_value_length: ByteRestriction::repeated(Limited(4)),
            constant_value: ByteRestriction::repeated(Undefined),
            constant_type: ByteRestriction::repeated(Limited(2)),
            variable_total_length: ByteRestriction::once(Limited(6)),
            variable_name_length: ByteRestriction::repeated(Limited(4)),
            variable_name: ByteRestriction::repeated(Undefined),
            variable_relationship: ByteRestriction::repeated(Limited(4)),
            precompiled_total_length: ByteRestriction::once(Limited(8)),
            precompiled_seg_length: ByteRestriction::repeated(Limited(6)),
            precompiled_code: ByteRestriction::repeated(Undefined),
            instructions_total_size: ByteRestriction::repeated(Limited(6)),
            instruction_line_length: ByteRestriction::repeated(Limited(4)),
            instruction_line: ByteRestriction::repeated(Undefined),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name_length.consume_bytes(self.name.consume_bytes(name.to_hex(0)).expect("name too long.").to_hex(128)).expect("name too long.");
    }

    pub fn add_constant(&mut self, hex: Hexadecimal, type_number: u16) {
        self.constant_value_length.consume_bytes((self.constant_value.consume_bytes(hex).expect("Constant value caused too many_bytes").to_hex(128))).expect("Constant caused too many bytes");
        self.constant_type.consume_bytes(type_number.to_hex(2)).unwrap();
    }

    pub fn add_variable(&mut self, name: String, relationship: u32) {
        self.variable_name_length.consume_bytes((self.variable_name.consume_bytes(name.to_hex(0)).expect("Variable name caused too many bytes").to_hex(128))).expect("Variable caused too many bytes");
        self.variable_relationship.consume_bytes(relationship.to_hex(4)).unwrap();
    }

    pub fn add_precompiled<T: Hexable>(&mut self, item: T) {
        self.precompiled_seg_length.consume_bytes(self.precompiled_code.consume_bytes(item.to_hex(0)).expect("Precompiled code too large").to_hex(128)).expect("Precompiled code too large");
    }

    pub fn add_instruction_line(&mut self, hex: Hexadecimal) {
        self.instruction_line_length.consume_bytes(self.instruction_line.consume_bytes(hex).expect("Hex line too long").to_hex(128)).expect("instruction line too long");

    }

    pub fn instruction_lines(&mut self) -> Vec<InstructionBytecode> {
        return self.instruction_line.iter_mut().unwrap().map(|b| InstructionBytecode::from(b)).collect::<Vec<InstructionBytecode>>();
    }

    pub fn from(hex: &mut Hexadecimal) -> FunctionBytecode {
        let mut fb = FunctionBytecode::new();
        let name_length = hex.scrape(fb.name_length.unpack().unwrap() as usize);
        let name_length_u32 = name_length.to_hexable::<u32>();
        let name = hex.scrape(name_length_u32 as usize);
        let mut constants_total_length = hex.scrape(fb.constants_total_length.unpack().unwrap() as usize);
        constants_total_length.extend_to_length(8); // we store as a 6 bytes but need to convert to u64
        let constants_total_length_u64 = constants_total_length.to_hexable::<u64>();
        let mut constants = hex.scrape(constants_total_length_u64 as usize);
        while !constants.is_empty() {
            let constant_value_length = constants.scrape(fb.constant_value_length.unpack().unwrap() as usize);
            let constant_value_length_u32 = constant_value_length.to_hexable::<u32>();
            let constant_value = constants.scrape(constant_value_length_u32 as usize);
            let constant_type = constants.scrape(fb.constant_type.unpack().unwrap() as usize);
            fb.constant_value_length.consume_bytes(constant_value_length).unwrap();
            fb.constant_value.consume_bytes(constant_value).unwrap();
            fb.constant_type.consume_bytes(constant_type).unwrap();
        }

        let mut variable_total_length = hex.scrape(fb.variable_total_length.unpack().unwrap() as usize);
        variable_total_length.extend_to_length(8); // we store as a 6 bytes but need to convert to u64
        let variable_total_length_u64 = variable_total_length.to_hexable::<u64>();
        let mut variables = hex.scrape(variable_total_length_u64 as usize);
        while !variables.is_empty() {
            let variable_name_length = variables.scrape(fb.variable_name_length.unpack().unwrap() as usize);
            let variable_name_length_u32 = variable_name_length.to_hexable::<u32>();
            let variable_name = variables.scrape(variable_name_length_u32 as usize);
            let variable_relationship = variables.scrape(fb.variable_relationship.unpack().unwrap() as usize);
            fb.variable_name_length.consume_bytes(variable_name_length).unwrap();
            fb.variable_name.consume_bytes(variable_name).unwrap();
            fb.variable_relationship.consume_bytes(variable_relationship).unwrap();
        }

        let precompiled_total_length = hex.scrape(fb.precompiled_total_length.unpack().unwrap() as usize);
        let precompiled_total_length_u64 = precompiled_total_length.to_hexable::<u64>();
        let mut precompiled = hex.scrape(precompiled_total_length_u64 as usize);
        while !precompiled.is_empty() {
            let mut precompiled_seg_length = precompiled.scrape(fb.precompiled_seg_length.unpack().unwrap() as usize);
            precompiled_seg_length.extend_to_length(8);
            let precompiled_seg_length_u64 = precompiled_seg_length.to_hexable::<u64>();
            let precompiled_code = precompiled.scrape(precompiled_seg_length_u64 as usize);
            fb.precompiled_seg_length.consume_bytes(precompiled_seg_length).unwrap();
            fb.precompiled_code.consume_bytes(precompiled_code).unwrap();
        }

        let mut instructions_total_size = hex.scrape(fb.instructions_total_size.unpack().unwrap() as usize);
        instructions_total_size.extend_to_length(8);
        let instruction_total_size_u64 = instructions_total_size.to_hexable::<u64>();
        let mut instructions = hex.scrape(instruction_total_size_u64 as usize);
        while !instructions.is_empty() {
            let instruction_line_length = instructions.scrape(fb.instruction_line_length.unpack().unwrap() as usize);
            let instruction_line_length_u32 = instruction_line_length.to_hexable::<u32>();
            let instruction_line = instructions.scrape(instruction_line_length_u32 as usize);

            fb.instruction_line_length.consume_bytes(instruction_line_length).unwrap();
            fb.instruction_line.consume_bytes(instruction_line).unwrap();
        }

        fb.name.consume_bytes(name).unwrap();
        fb.name_length.consume_bytes(name_length).unwrap();

        return fb;
    }
}

impl ToBytecode for FunctionBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let instruction_bytes = self.instruction_line_length.join(&self.instruction_line);
        self.instructions_total_size.consume_bytes(instruction_bytes.len().to_hex(128)).expect("Instructions too long");

        let precompile = self.precompiled_seg_length.join(&self.precompiled_code);
        self.precompiled_total_length.consume_bytes(precompile.len().to_hex(128)).expect("Precompiled code too long");

        let variables = self.variable_name_length.join_thrice(&self.variable_name, &self.variable_relationship);
        self.variable_total_length.consume_bytes(variables.len().to_hex(128)).expect("Variables too long");

        let constants = self.constant_value_length.join_thrice(&self.constant_value, &self.constant_type);
        self.constants_total_length.consume_bytes(constants.len().to_hex(128)).expect("Constants too long");



        return self.name_length.bytes() + self.name.bytes() + self.constants_total_length.bytes() + constants +
            self.variable_total_length.bytes() + variables + self.precompiled_total_length.bytes() + precompile + self.instructions_total_size.bytes() + instruction_bytes;
    }
}