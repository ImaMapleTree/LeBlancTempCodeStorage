use core::slice::Iter;
use crate::leblanc::core::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::core::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::bytecode::ToBytecode;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

#[derive(Debug)]
pub struct FileBodyBytecode {
    total_body_size: ByteRestriction,
    function_size: ByteRestriction,
    function: ByteRestriction,
}

impl FileBodyBytecode {
    pub fn new() -> FileBodyBytecode {
        return FileBodyBytecode {
            total_body_size: ByteRestriction::once(Limited(8)),
            function_size: ByteRestriction::repeated(Limited(8)),
            function: ByteRestriction::repeated(Undefined)
        }
    }

    pub fn add_function(&mut self, mut function: FunctionBytecode) {
        self.function_size.consume_bytes(self.function.consume_bytes(function.generate()).expect("Function too large").to_hex(128)).expect("Function too large");
    }

    pub fn from(hex: &mut Hexadecimal) -> FileBodyBytecode {
        let mut body = FileBodyBytecode::new();
        let total_body_size = hex.scrape(body.total_body_size.unpack().unwrap() as usize);
        let total_body_size_u64 = u64::from_hex(&total_body_size);
        let mut body_bytes = hex.scrape(total_body_size_u64 as usize);
        while !body_bytes.is_empty() {
            let function_size = body_bytes.scrape(body.function_size.unpack().unwrap() as usize);
            let function_size_u64 = u64::from_hex(&function_size);
            let function = body_bytes.scrape(function_size_u64 as usize);
            body.function_size.consume_bytes(function_size).unwrap();
            body.function.consume_bytes(function).unwrap();
        }

        body.total_body_size.consume_bytes(total_body_size).unwrap();
        return body;

    }

    pub fn functions(&mut self) -> Vec<FunctionBytecode> {
        return self.function.iter_mut().unwrap().map(|f| FunctionBytecode::from(f)).collect::<Vec<FunctionBytecode>>();
    }


}

impl ToBytecode for FileBodyBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let functions = self.function_size.join_uncloned(&mut self.function);
        self.total_body_size.consume_bytes(functions.len().to_hex(128)).unwrap();
        let mut body_size = self.total_body_size.bytes();
        body_size.consume(functions);
        //self.total_body_size.consume_bytes(functions).unwrap(); // This looks weird but we do it so that we no longer clone the functions obj saving on some storage space
        return body_size;
    }
}