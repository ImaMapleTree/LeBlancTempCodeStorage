use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use crate::leblanc::compiler::lalrpop;
use crate::leblanc::compiler::parser::ast::{Component, push_byte_location, Required, set_file};
use crate::leblanc::rustblanc::lb_file::{LBFile, LBFileTrait, LBVirtualFile};
use crate::leblanc::rustblanc::path::ZCPath;

#[derive(Clone, Hash, Default, PartialEq, Eq, Debug)]
pub struct CompileModule {
    file: LBFile,
    content: String
}

impl CompileModule {
    pub fn new(mut file: LBFile) -> CompileModule {
        let content = if !file.is_dir() {
            file.read().unwrap()
        } else { "".to_string() };
        CompileModule { file, content }
    }

    fn prep_parser(&self) {
        unsafe { set_file(self.file.path()) }
        let bytes = self.content.as_bytes();

        let mut line_number: usize = 1;
        let mut symbol_number: usize = 0;
        for byte in bytes {
            unsafe { push_byte_location((line_number, symbol_number)); }
            symbol_number += 1;
            if *byte == 10 {
                line_number += 1;
                symbol_number = 0;
            }
        }
    }

    pub fn parse_components(&self) -> Result<Vec<Component>, ParseError<usize, Token<'_>, &str>> {
        self.prep_parser();
        lalrpop::FileParser::new().parse(&self.content)
    }

    pub fn parse_required(&self) -> Result<Vec<Required>, ParseError<usize, Token<'_>, &str>> {
        self.prep_parser();
        lalrpop::SkeletonFileParser::new().parse(&self.content)
    }

    pub fn is_skeleton(&self) -> bool {
        self.file.path().to_string().starts_with(".skeleton")
    }

    pub fn path(&self) -> ZCPath {
        self.file.path()
    }
}

