use crate::leblanc::core::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::core::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::core::bytecode::ToBytecode;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

#[derive(Debug)]
pub struct ExtensionBytecode {
    name_length: ByteRestriction,
    name: ByteRestriction,
    total_parameter_length: ByteRestriction,
    parameter_name_length: ByteRestriction, // for adding variables to objects
    parameter_name: ByteRestriction,
    parameter_type: ByteRestriction,
    total_method_length: ByteRestriction,
    owned_method_index: ByteRestriction,
}

impl ExtensionBytecode {
    pub fn new() -> ExtensionBytecode {
        return ExtensionBytecode {
            name_length: ByteRestriction::once(Limited(4)),
            name: ByteRestriction::once(Undefined),
            total_parameter_length: ByteRestriction::once(Limited(4)),
            parameter_name_length: ByteRestriction::repeated(Limited(4)),
            parameter_name: ByteRestriction::repeated(Undefined),
            parameter_type: ByteRestriction::repeated(Limited(2)),
            total_method_length: ByteRestriction::once(Limited(4)),
            owned_method_index: ByteRestriction::repeated(Limited(4))
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name_length.consume_bytes(self.name.consume_bytes(name.to_hex(0)).expect("name too long.").to_hex(128)).expect("name too long.");
    }

    pub fn add_parameter(&mut self, name: String, type_hex: Hexadecimal) {
        self.parameter_name_length.consume_bytes(self.parameter_name.consume_bytes(name.to_hex(0)).expect("name too long").to_hex(128)).expect("name too long");
        self.parameter_type.consume_bytes(type_hex).expect("type hex too much");
    }

    pub fn add_reference_method(&mut self, method_index: u32) {
        self.total_method_length.consume_bytes(self.owned_method_index.consume_bytes(method_index.to_hex(4)).expect("method bytes too long").to_hex(128)).expect("method bytes too long");
    }

    pub fn from(hex: &mut Hexadecimal) -> ExtensionBytecode {
        let mut bytecode = ExtensionBytecode::new();
        let name_length = hex.scrape(bytecode.name_length.unpack().unwrap() as usize);
        let name_length_u32 = name_length.to_hexable::<u32>();
        let name = hex.scrape(name_length_u32 as usize);
        let total_parameter_length = hex.scrape(bytecode.total_parameter_length.unpack().unwrap() as usize);
        let total_parameter_length_u32 = total_parameter_length.to_hexable::<u32>();
        let mut parameter_bytes = hex.scrape(total_parameter_length_u32 as usize);
        while !parameter_bytes.is_empty() {
            let parameter_name_length = parameter_bytes.scrape(bytecode.parameter_name_length.unpack().unwrap() as usize);
            let parameter_name_length_u32 = parameter_name_length.to_hexable::<u32>();
            let parameter_name = parameter_bytes.scrape(parameter_name_length_u32 as usize);
            let parameter_type = parameter_bytes.scrape(bytecode.parameter_type.unpack().unwrap() as usize);
            bytecode.parameter_name_length.consume_bytes(parameter_name_length).unwrap();
            bytecode.parameter_name.consume_bytes(parameter_name).unwrap();
            bytecode.parameter_type.consume_bytes(parameter_type).unwrap();
        }
        let total_method_length = hex.scrape(bytecode.total_method_length.unpack().unwrap() as usize);
        let total_method_length_u32 = total_method_length.to_hexable::<u32>();
        let mut method_bytes = hex.scrape(total_method_length_u32 as usize);
        while !method_bytes.is_empty() {
            let method_index = method_bytes.scrape(bytecode.owned_method_index.unpack().unwrap() as usize);
            bytecode.owned_method_index.consume_bytes(method_index).unwrap();
        }

        bytecode.name_length.consume_bytes(name_length).unwrap();
        bytecode.name.consume_bytes(name).unwrap();
        bytecode.total_parameter_length.consume_bytes(total_parameter_length).unwrap();
        bytecode.total_method_length.consume_bytes(total_method_length).unwrap();

        return bytecode;
    }

}

impl ToBytecode for ExtensionBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let parameter_bytes = self.parameter_name_length.join_thrice(&self.parameter_name, &self.parameter_type);
        self.total_parameter_length.consume_bytes(parameter_bytes.len().to_hex(128)).expect("Parameter too long");

        return self.name_length.bytes() + self.name.bytes() + self.total_parameter_length.bytes() + parameter_bytes + self.total_method_length.bytes() + self.owned_method_index.bytes();

    }
}