use std::collections::HashMap;
use std::mem::take;
use std::sync::Arc;
use crate::leblanc::compiler3::symbols::byte_layout::{BYTE_BOUNDARY, REPEAT_START};
use crate::leblanc::compiler3::symbols::byte_layout::LayoutKey::{Boundary, RepeatStart};
use crate::leblanc::compiler3::symbols::cst::{LCST, SymbolType};

#[derive(Default)]
pub struct BytecodeGenerator {
    constants: Vec<u8>,
    functions: HashMap<u64, (Vec<u8>, Vec<u8>)>,
    classes: HashMap<u64, (Vec<u8>, Vec<u8>)>,
}

impl BytecodeGenerator {
    pub fn constants(&mut self) -> &mut Vec<u8> {
        &mut self.constants
    }

    pub fn functions(&mut self, id: u64) -> &mut (Vec<u8>, Vec<u8>) {
        self.functions.entry(id).or_insert_with(|| (vec![], vec![]));
        self.functions.get_mut(&id).unwrap()
    }

    pub fn classes(&mut self, id: u64) -> &mut (Vec<u8>, Vec<u8>) {
        self.classes.entry(id).or_insert_with(|| (vec![], vec![]));
        self.classes.get_mut(&id).unwrap()
    }
}


pub fn generate_bytecode(table: LCST) {
    let mut generator = BytecodeGenerator::default();
    let table = take(unsafe { &mut *table.data_ptr()});
    println!("Table: {:?}", table);
    for symbol in table.symbols {
        println!("Symbol: {:?}", symbol);
        match symbol.stype {
            SymbolType::Function(id) => {
                let (meta, code) = generator.functions(id);
                meta.append(&mut symbol.name.as_bytes());
                meta.push(BYTE_BOUNDARY);
                meta.extend_from_slice(&(symbol.subsymbols.len() as u32).to_le_bytes());
                meta.extend_from_slice(&(symbol.linked.unwrap().lock().shared.as_ref().map(|s| s.lock().len()).unwrap_or(0) as u32).to_le_bytes());
                meta.push(REPEAT_START);
                for subsymbol in symbol.subsymbols {
                    meta.extend_from_slice(&subsymbol.typ.unwrap().to_bytes());
                    meta.append(&mut subsymbol.name.as_bytes());
                    meta.push(BYTE_BOUNDARY);
                }
                println!("Bytes: {:?}", meta);
            }
            SymbolType::Class(id) => {}
            SymbolType::Standard => {}
        }
    }
}