use crate::leblanc::core::bytecode::byte_limiter::ByteRestriction;
use crate::leblanc::core::bytecode::byte_limiter::ByteLimit::{Limited, Undefined};
use crate::leblanc::core::bytecode::ToBytecode;
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::lib::datetime::date_as_hex;

///    XX     XX   XXXXXXXXX   XXXXXXXXX   XXXXXXXX     XXXXXXXXX   XXXXXXX
///    XX     XX   XX          XX     XX   XX     XX    XX          XX     XX
///    XXXXXXXXX   XXXXXX      XXXXXXXXX   XX     XX    XXXXXX      XXXXXXXXX
///    XX     XX   XX          XX     XX   XX     XX    XX          XX    XX
///    XX     XX   XXXXXXXXX   XX     XX   XXXXXXXX     XXXXXXXXX   XX     XX
///
///     Date
///     year (2 bytes) | month (1 byte) | day (1 byte) | hour (1 byte) | minute (1 byte) | second (1 byte) |
///
///
///     total_file_size | date_modified | total_header_size | import_size | import-length | import-name |          |
///         8 bytes    |     6 bytes   |      4 bytes      |   4 bytes   |    4 bytes    |    X bytes  | ........ |
///
///       global_size   |  gname_size   |   global_name     | ........... |
///         4 bytes     |    4 bytes    |      X bytes      | ........... |
///


#[derive(Debug)]
pub struct FileHeaderBytecode {
    date_modified: ByteRestriction,
    total_header_size: ByteRestriction,
    import_size: ByteRestriction,
    import_name_length: ByteRestriction,
    import_name: ByteRestriction,
    global_size: ByteRestriction,
    global_name_length: ByteRestriction,
    global_name: ByteRestriction
}

impl FileHeaderBytecode {
    pub fn new() -> FileHeaderBytecode {
        return FileHeaderBytecode {
            date_modified: ByteRestriction::once(Limited(8)),
            total_header_size: ByteRestriction::once(Limited(4)),
            import_size: ByteRestriction::once(Limited(4)),
            import_name_length: ByteRestriction::repeated(Limited(4)),
            import_name: ByteRestriction::repeated(Undefined),
            global_size: ByteRestriction::once(Limited(4)),
            global_name_length: ByteRestriction::repeated(Limited(4)),
            global_name: ByteRestriction::repeated(Undefined)
        }
    }

    pub fn from(hex: &mut Hexadecimal) -> FileHeaderBytecode {
        let mut header = FileHeaderBytecode::new();
        let date_modified = hex.scrape(header.date_modified.unpack().unwrap() as usize);
        let total_header_size = hex.scrape(header.total_header_size.unpack().unwrap() as usize);
        let import_size = hex.scrape(header.import_size.unpack().unwrap() as usize);
        let import_size_u32 = u32::from_hex(&import_size);
        let mut imports = hex.scrape(import_size_u32 as usize);

        while !imports.is_empty() {
            let import_name_length = imports.scrape(header.import_name_length.unpack().unwrap() as usize);
            let import_name_length_u32 = u32::from_hex(&import_name_length);
            let import_name = imports.scrape(import_name_length_u32 as usize);
            header.import_name_length.consume_bytes(import_name_length).unwrap();
            header.import_name.consume_bytes(import_name).unwrap();
        }

        let global_size = hex.scrape(header.global_size.unpack().unwrap() as usize);
        let global_size_u32 = u32::from_hex(&global_size);
        let globals = hex.scrape(global_size_u32 as usize);

        while !globals.is_empty() {
            let global_name_length = imports.scrape(header.import_name_length.unpack().unwrap() as usize);
            let global_name_length_32 = u32::from_hex(&global_name_length);
            let global_name = imports.scrape(global_name_length_32 as usize);
            header.global_name_length.consume_bytes(global_name_length).unwrap();
            header.global_name.consume_bytes(global_name).unwrap();
        }

        header.date_modified.consume_bytes(date_modified).unwrap();
        header.total_header_size.consume_bytes(total_header_size).unwrap();
        header.import_size.consume_bytes(import_size).unwrap();
        header.global_size.consume_bytes(global_size).unwrap();

        return header;


    }

    pub fn add_import_name(&mut self, name: &String) {
        self.import_name_length.consume_bytes((self.import_name.consume_bytes(name.to_hex(1024)).unwrap() as u32).to_hex(4)).expect("Import name caused too many bytes");
    }

    pub fn add_global_name(&mut self, name: &String) {
        self.global_name_length.consume_bytes((self.global_name.consume_bytes(name.to_hex(1024)).unwrap() as u32).to_hex(4)).expect("Global name caused too many bytes");
    }
}

impl ToBytecode for FileHeaderBytecode {
    fn generate(&mut self) -> Hexadecimal {
        let import_bytes = self.import_name_length.join(&self.import_name);
        self.import_size.consume_bytes((import_bytes.len() as u32).to_hex(4)).expect("Total import length causes too many bytes");
        let global_bytes = self.global_name_length.join(&self.global_name);
        self.global_size.consume_bytes((global_bytes.len() as u32).to_hex(4)).expect("Total global length causes too many bytes");
        self.total_header_size.consume_bytes(((import_bytes.len() + global_bytes.len()) as u64).to_hex(4)).unwrap();
        self.date_modified.consume_bytes(date_as_hex()).expect("Date time caused too many bytes");
        return self.date_modified.bytes() + self.total_header_size.bytes() + self.import_size.bytes() + import_bytes + self.global_size.bytes() + global_bytes;

    }
}