use std::fs::File;
use crate::{BraceOpen, CompileVocab, create_stack, create_tokens, error_report, leblanc, PartialFabric, Semicolon, TypedToken};
use crate::leblanc::compiler::char_reader::CharReader;
use crate::leblanc::compiler::compile_types::CompilationMode;
use crate::leblanc::compiler::compile_types::stub_compiler::{create_stub_dump, read_from_stub_dump};
use crate::leblanc::compiler::compiler_util::flatmap_node_tokens;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;

static DEBUG: bool = false;

pub fn compile(string: String, mode: CompilationMode) -> PartialFabric {
    let filesf_name = string.replace(".lb", ".lbsf");
    let filesf = File::open(filesf_name);
    let mut stub_file_exists = false;
    let mut fabric = PartialFabric::no_path(vec![], vec![], vec![]);
    if filesf.is_ok() {
        fabric = read_from_stub_dump(filesf.unwrap());
        stub_file_exists = true;
    }

    if mode == CompilationMode::Realtime {
        let mut cr = CharReader::from_line(string);
        fabric = partial_spin(&mut cr ,mode);
    }

    else if fabric.is_null() {
        let f = File::open(string).unwrap();
        let mut cr = CharReader::new(f);
        fabric = partial_spin(&mut cr, mode);
    }

    if mode == CompilationMode::StubFile {
        if !stub_file_exists {
            //create_stub_dump(&mut fabric);
        }
        return fabric;
    } else {
        create_execution_stack(&mut fabric);
    }

    //("test.lbsf".to_string());
    return fabric;
}

// haha I'm so hip because I call my methods fancy things
// here we're "spinning" the "fabric"
// 😎 (Sunglasses emoji)
pub fn partial_spin(cr: &mut CharReader, mode: CompilationMode) -> PartialFabric {
    let mut fabric = create_tokens(cr, mode);


    if DEBUG {
        println!("imports: {:?}", fabric.imports());
        for token in fabric.tokens() {
            println!("{:?}", token.value);
        }
    }


    if fabric.errors().len() > 0 {
        error_report(cr, &fabric.tokens().iter().cloned().map(|t| t.value.clone()).collect(), fabric.errors());
    }

    return fabric;


}

pub fn create_execution_stack(fabric: &mut PartialFabric) -> Vec<TypedToken> {
    let stack: Vec<TypedToken> = Vec::new();
    let mut boundary_index = fabric.tokens().iter().enumerate().filter(|(_, r)| r.value.lang_type() == CompileVocab::BOUNDARY(Semicolon) ||
        r.value.lang_type() == CompileVocab::BOUNDARY(BraceOpen))
        .map(|(index, _)| index+1)
        .collect::<Vec<_>>();
    boundary_index.insert(0, 0);
    boundary_index.insert(boundary_index.len(), fabric.tokens().len());
    for i in 0..boundary_index.len()-1 {
        println!("-----------");
        let mut mini_stack = Vec::new();
        if fabric.tokens()[boundary_index[i]].value.lang_type() == CompileVocab::KEYWORD(LBKeyword::Func) {
            for token in flatmap_node_tokens(&mut fabric.tokens()[boundary_index[i]..boundary_index[i + 1]].to_vec()) { mini_stack.insert(mini_stack.len(), token) }
        } else {
            create_stack(&mut fabric.tokens()[boundary_index[i]..boundary_index[i + 1]].to_vec(), &mut mini_stack);
            mini_stack.reverse();
        }
        for token in mini_stack {
            if token.lang_type().matches("boundary") { println!("{:?}", token); }
            else {
                println!("{:?}", token);
            }
        }
    }

    let mut stack = Vec::new();
    create_stack(fabric.tokens(), &mut stack);
    stack.reverse();
    return stack;
}