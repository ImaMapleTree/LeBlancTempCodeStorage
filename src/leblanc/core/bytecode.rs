pub mod file_header;
pub mod byte_limiter;
pub mod file_body;
pub mod function_bytes;
pub mod instruction_line_bytes;
pub mod precompiled;
pub mod decompiled_constant;
pub mod extension_bytes;

use crate::leblanc::core::bytecode::file_body::FileBodyBytecode;
use crate::leblanc::core::bytecode::file_header::FileHeaderBytecode;
use crate::leblanc::rustblanc::hex::Hexadecimal;

///    XX     XX   XXXXXXXXX   XXXXXXXXX   XXXXXXXX     XXXXXXXXX   XXXXXXX
///    XX     XX   XX          XX     XX   XX     XX    XX          XX     XX
///    XXXXXXXXX   XXXXXX      XXXXXXXXX   XX     XX    XXXXXX      XXXXXXXXX
///    XX     XX   XX          XX     XX   XX     XX    XX          XX    XX
///    XX     XX   XXXXXXXXX   XX     XX   XXXXXXXX     XXXXXXXXX   XX     XX
///
///     total_file_size | date_modified | total_header_size | import_size | import-length | import-name |          |
///         12 bytes    |     ? bytes   |      4 bytes      |   4 bytes   |    4 bytes    |    X bytes  | ........ |
///
///       global_size   |  gname_size   |   global_name     | ........... |
///         4 bytes     |    4 bytes    |      X bytes      | ........... |
///
///
///
///
///
///
///
///
///

pub trait ToBytecode {
    fn generate(&mut self) -> Hexadecimal;
}



#[derive(Debug)]
pub struct LeblancBytecode {
    file_header: FileHeaderBytecode,
    body: FileBodyBytecode
}

impl LeblancBytecode {
    pub fn new(file_header: FileHeaderBytecode, body: FileBodyBytecode) -> LeblancBytecode {
        LeblancBytecode {
            file_header,
            body
        }
    }


    pub fn from(mut hex: Hexadecimal) -> LeblancBytecode {
        let file_header = FileHeaderBytecode::from(&mut hex);
        let body = FileBodyBytecode::from(&mut hex);
        LeblancBytecode::new(file_header, body)
    }

    pub fn file_header(&mut self) -> &mut FileHeaderBytecode { &mut self.file_header }

    pub fn body(&mut self) -> &mut FileBodyBytecode { &mut self.body }




}

impl ToBytecode for LeblancBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let mut header_bytes = self.file_header.generate();
        header_bytes.consume( self.body.generate());
        header_bytes
    }
}