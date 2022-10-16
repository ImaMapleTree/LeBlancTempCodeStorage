use std::collections::HashMap;
use fxhash::{FxHashMap};
use crate::leblanc::compiler::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::compiler::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::compiler::bytecode::decompiled_constant::DecompiledConstant;
use crate::leblanc::compiler::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::compiler::bytecode::ToBytecode;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Debug)]
pub struct FunctionBytecode {
    name_length: ByteRestriction,
    name: ByteRestriction,
    argument_length: ByteRestriction,
    arguments: ByteRestriction,
    returns_length: ByteRestriction,
    returns: ByteRestriction,
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

impl Default for FunctionBytecode {
    fn default() -> Self {
        FunctionBytecode::new()
    }
}

impl FunctionBytecode {
    pub fn new() -> FunctionBytecode {
        FunctionBytecode {
            name_length: ByteRestriction::once(Limited(4)),
            name: ByteRestriction::once(Undefined),
            argument_length: ByteRestriction::once(Limited(4)),
            arguments: ByteRestriction::repeated(Limited(2)),
            returns_length: ByteRestriction::once(Limited(4)),
            returns: ByteRestriction::repeated(Limited(2)),
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

    pub fn name(&self) -> String {
        self.name.bytes().to_hexable::<String>()
    }

    pub fn add_argument(&mut self, leblanc_type: LeBlancType) {
        self.arguments.consume_bytes(leblanc_type.enum_id().to_hex(2)).expect("Argument too long").to_hex(128);
    }

    pub fn add_return(&mut self, leblanc_type: LeBlancType) {
        self.returns.consume_bytes(leblanc_type.enum_id().to_hex(2)).expect("Type too long");
    }


    pub fn add_constant(&mut self, hex: Hexadecimal, type_number: u16) {
        self.constant_value_length.consume_bytes(self.constant_value.consume_bytes(hex).expect("Constant value caused too many_bytes").to_hex(128)).expect("Constant caused too many bytes");
        self.constant_type.consume_bytes(type_number.to_hex(2)).unwrap();
    }


    pub fn add_variable(&mut self, name: String, relationship: u32) {
        self.variable_name_length.consume_bytes(self.variable_name.consume_bytes(name.to_hex(0)).expect("Variable name caused too many bytes").to_hex(128)).expect("Variable caused too many bytes");
        self.variable_relationship.consume_bytes(relationship.to_hex(4)).unwrap();
    }

    pub fn add_precompiled<T: Hexable>(&mut self, item: T) {
        self.precompiled_seg_length.consume_bytes(self.precompiled_code.consume_bytes(item.to_hex(0)).expect("Precompiled code too large").to_hex(128)).expect("Precompiled code too large");
    }

    pub fn add_instruction_line(&mut self, hex: Hexadecimal) {
        self.instruction_line_length.consume_bytes(self.instruction_line.consume_bytes(hex).expect("Hex line too long").to_hex(128)).expect("instruction line too long");

    }

    pub fn instruction_lines(&mut self) -> Vec<InstructionBytecode> {
        return self.instruction_line.iter_mut().unwrap().map(InstructionBytecode::from).collect::<Vec<InstructionBytecode>>();
    }

    pub fn constants(&mut self) -> Vec<DecompiledConstant> {
        let mut constants = vec![];
        let constant_length = self.constant_value.segments().unwrap().len();
        for _ in 0..constant_length {
            let constant_value = self.constant_value.remove(0).unwrap();
            let constant_type = self.constant_type.remove(0).unwrap();
            constants.push(DecompiledConstant::new(constant_value, LeBlancType::from_enum_id(constant_type.to_hexable::<u16>())))
        }
        constants
    }

    pub fn variables(&mut self) -> HashMap<String, VariableContext> {
        let mut variables: HashMap<String, VariableContext> = HashMap::default();
        let variable_length = self.variable_name.segments().unwrap().len();
        for _ in 0..variable_length {
            let variable_name = self.variable_name.remove(0).unwrap().to_hexable::<String>();
            let variable_relationship = self.variable_relationship.remove(0).unwrap();
            variables.insert(variable_name.clone(), VariableContext::shell(variable_name, variable_relationship.to_hexable::<u32>()));
        }
        variables
    }

    pub fn arguments(&mut self) -> Vec<LeBlancType> {
        self.arguments.iter().unwrap().map(|hex| LeBlancType::from_enum_id(hex.to_hexable::<u16>())).collect()
    }

    pub fn from(hex: &mut Hexadecimal) -> FunctionBytecode {
        let mut fb = FunctionBytecode::new();
        let name_length = hex.scrape(fb.name_length.unpack().unwrap() as usize);
        let name_length_u32 = name_length.to_hexable::<u32>();
        let name = hex.scrape(name_length_u32 as usize);
        let argument_length = hex.scrape(fb.argument_length.unpack().unwrap() as usize);
        let argument_length_u32 = argument_length.to_hexable::<u32>();
        let mut arguments = hex.scrape(argument_length_u32 as usize);
        while !arguments.is_empty() {
            let argument = arguments.scrape(fb.arguments.unpack().unwrap() as usize);
            fb.arguments.consume_bytes(argument).unwrap();
        }
        let returns_length = hex.scrape(fb.returns_length.unpack().unwrap() as usize);
        let returns_length_u32 = returns_length.to_hexable::<u32>();
        let mut returns = hex.scrape(returns_length_u32 as usize);
        while !returns.is_empty() {
            let return_type = returns.scrape(fb.returns.unpack().unwrap() as usize);
            fb.returns.consume_bytes(return_type).unwrap();
        }

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
        println!("NAME: {}", String::from_hex(&name));

        println!("PRECOMPILED TOTAL LENGTH: {:?}", precompiled_total_length_u64);
        let mut instructions_total_size = hex.scrape(fb.instructions_total_size.unpack().unwrap() as usize);
        instructions_total_size.extend_to_length(8);
        let instruction_total_size_u64 = instructions_total_size.to_hexable::<u64>();
        println!("INSTRUCTION TOTAL LENGTH: {:?}", instruction_total_size_u64);
        let mut instructions = hex.scrape(instruction_total_size_u64 as usize);
        while !instructions.is_empty() {
            let instruction_line_length = instructions.scrape(fb.instruction_line_length.unpack().unwrap() as usize);
            let instruction_line_length_u32 = instruction_line_length.to_hexable::<u32>();
            let instruction_line = instructions.scrape(instruction_line_length_u32 as usize);

            fb.instruction_line_length.consume_bytes(instruction_line_length).unwrap();
            fb.instruction_line.consume_bytes(instruction_line).unwrap();
        }

        fb.argument_length.consume_bytes(argument_length).unwrap();
        fb.name.consume_bytes(name).unwrap();
        fb.name_length.consume_bytes(name_length).unwrap();

        fb
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


        self.argument_length.consume_bytes(self.arguments.bytes().len().to_hex(4)).expect("arguments too long");

        self.returns_length.consume_bytes(self.returns.bytes().len().to_hex(4)).expect("returns too long");

        self.name_length.bytes() + self.name.bytes() + self.argument_length.bytes() + self.arguments.bytes() + self.returns_length.bytes() + self.returns.bytes() + self.constants_total_length.bytes() + constants +
            self.variable_total_length.bytes() + variables + self.precompiled_total_length.bytes() + precompile + self.instructions_total_size.bytes() + instruction_bytes
    }
}