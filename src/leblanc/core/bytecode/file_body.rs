
use crate::leblanc::core::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::core::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::core::bytecode::extension_bytes::ExtensionBytecode;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::ToBytecode;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

#[derive(Debug)]
pub struct FileBodyBytecode {
    total_function_size: ByteRestriction,
    function_size: ByteRestriction,
    function: ByteRestriction,
    total_extension_size: ByteRestriction,
    extension_size: ByteRestriction,
    extension: ByteRestriction,
}

impl FileBodyBytecode {
    pub fn new() -> FileBodyBytecode {
        FileBodyBytecode {
            total_function_size: ByteRestriction::once(Limited(8)),
            function_size: ByteRestriction::repeated(Limited(8)),
            function: ByteRestriction::repeated(Undefined),
            total_extension_size: ByteRestriction::once(Limited(8)),
            extension_size: ByteRestriction::repeated(Limited(8)),
            extension: ByteRestriction::repeated(Undefined)
        }
    }

    pub fn add_function(&mut self, mut function: FunctionBytecode) {
        self.function_size.consume_bytes(self.function.consume_bytes(function.generate()).expect("Function too large").to_hex(128)).expect("Function too large");
    }

    pub fn add_extension(&mut self, mut extension: ExtensionBytecode) {
        self.extension_size.consume_bytes(self.extension.consume_bytes(extension.generate()).expect("Extension too large").to_hex(128)).expect("Extension too large");
    }

    pub fn from(hex: &mut Hexadecimal) -> FileBodyBytecode {
        let mut body = FileBodyBytecode::new();

        let total_function_size = hex.scrape(body.total_function_size.unpack().unwrap() as usize);
        let total_function_size_u64 = u64::from_hex(&total_function_size);
        let mut function_bytes = hex.scrape(total_function_size_u64 as usize);
        while !function_bytes.is_empty() {
            let function_size = function_bytes.scrape(body.function_size.unpack().unwrap() as usize);
            let function_size_u64 = u64::from_hex(&function_size);
            let function = function_bytes.scrape(function_size_u64 as usize);
            body.function_size.consume_bytes(function_size).unwrap();
            body.function.consume_bytes(function).unwrap();
        }

        let total_extension_size = hex.scrape(body.total_extension_size.unpack().unwrap() as usize);
        let total_extension_size_u64 = total_extension_size.to_hexable::<u64>();
        let mut extension_bytes = hex.scrape(total_extension_size_u64 as usize);
        while !extension_bytes.is_empty() {
            let extension_size = extension_bytes.scrape(body.extension_size.unpack().unwrap() as usize);
            let extension_size_u64 = extension_size.to_hexable::<u64>();
            let extension = extension_bytes.scrape(extension_size_u64 as usize);
            body.extension_size.consume_bytes(extension_size).unwrap();
            body.extension.consume_bytes(extension).unwrap();
        }

        body

    }

    pub fn functions(&mut self) -> Vec<FunctionBytecode> {
        return self.function.iter_mut().unwrap().map(FunctionBytecode::from).collect::<Vec<FunctionBytecode>>();
    }

    pub fn extensions(&mut self) -> Vec<ExtensionBytecode> {
        return self.extension.iter_mut().unwrap().map(ExtensionBytecode::from).collect::<Vec<ExtensionBytecode>>();
    }



}

impl ToBytecode for FileBodyBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let functions = self.function_size.join_uncloned(&mut self.function);
        self.total_function_size.consume_bytes(functions.len().to_hex(128)).unwrap();

        let extensions = self.extension_size.join_uncloned(&mut self.extension);
        self.total_extension_size.consume_bytes(extensions.len().to_hex(128)).unwrap();

        let mut final_bytes = self.total_function_size.bytes();
        final_bytes.consume(functions);
        final_bytes.consume(self.total_extension_size.bytes());
        final_bytes.consume(extensions);

        //self.total_body_size.consume_bytes(functions).unwrap(); // This looks weird but we do it so that we no longer clone the functions obj saving on some storage space
        final_bytes
    }
}