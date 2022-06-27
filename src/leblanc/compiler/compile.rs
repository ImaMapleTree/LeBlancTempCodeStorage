use std::fs::File;
use crate::{BraceOpen, CompileVocab, create_stack, create_tokens, Fabric, Semicolon, TypedToken};
use crate::leblanc::compiler::char_reader::CharReader;
use crate::leblanc::compiler::compile_error_reporter::error_report;
use crate::leblanc::compiler::compile_types::CompilationMode;
use crate::leblanc::compiler::compile_types::full_compiler::write_bytecode;
use crate::leblanc::compiler::compile_types::stub_compiler::read_from_stub_dump;
use crate::leblanc::compiler::compiler_util::flatmap_node_tokens;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_lang::BoundaryType::BraceClosed;
use crate::leblanc::rustblanc::Appendable;

static DEBUG: bool = false;

pub fn compile(string: String, mode: CompilationMode) -> Fabric {
    println!("Compiling: {}", string);
    let filesf_name = string.replace(".lb", ".lbsf");
    let filesf = File::open(filesf_name);
    let mut stub_file_exists = false;
    let mut fabric = Fabric::no_path(vec![], vec![], vec![],vec![]);
    if filesf.is_ok() {
        fabric = read_from_stub_dump(filesf.unwrap());
        stub_file_exists = true;
    }

    if mode == CompilationMode::Realtime {
        let mut cr = CharReader::from_line(string);
        fabric = partial_spin(&mut cr ,mode);
        println!("Fabric: {:?}", fabric.tokens());
    }

    else if fabric.is_null() {
        let f = File::open(string).unwrap();
        let mut cr = CharReader::new(f);
        println!("Spin in");
        fabric = partial_spin(&mut cr, mode);
        println!("Spin out");
    }

    if mode == CompilationMode::StubFile {
        if !stub_file_exists {
            //create_stub_dump(&mut fabric);
        }
        return fabric;
    } else {

        let tokens = create_execution_stack(&mut fabric);


        write_bytecode(tokens, &mut fabric, mode);
    }

    //("test.lbsf".to_string());
    fabric
}

// haha I'm so hip because I call my methods fancy things
// here we're "spinning" the "fabric"
// 😎 (Sunglasses emoji)
pub fn partial_spin(cr: &mut CharReader, mode: CompilationMode) -> Fabric {
    let mut fabric = create_tokens(cr, mode);
    println!("Done creating tokens");

    if DEBUG {
        println!("imports: {:?}", fabric.imports());
        for token in fabric.tokens() {
            println!("{:?}", token.value);
        }
    }


    println!("Errors: {:?}", fabric.errors());

    if !fabric.errors().is_empty() {
        error_report(cr, &fabric.tokens().iter().cloned().map(|t| t.value.clone()).collect(), fabric.errors());
    }

    fabric


}

pub fn create_execution_stack(fabric: &mut Fabric) -> Vec<TypedToken> {
    let mut stack: Vec<TypedToken> = Vec::new();
    let token_length = fabric.tokens().len();
    let mut boundary_index = fabric.tokens().iter().enumerate().filter(|(_, r)| r.value.lang_type() == CompileVocab::BOUNDARY(Semicolon) ||
        r.value.lang_type() == CompileVocab::BOUNDARY(BraceOpen) || r.value.lang_type() == CompileVocab::BOUNDARY(BraceClosed))
        .map(|(index, _)| {
            if index+1 >= token_length {
                index
            } else {
                index + 1
            }
        })
        .collect::<Vec<_>>();
    boundary_index.insert(0, 0);
    boundary_index.append_item(fabric.tokens().len()-1);

    let stack_print = true;

    for i in 0..boundary_index.len()-1 {
        if stack_print {println!("-----------");}
        let mut mini_stack = Vec::new();
        if fabric.tokens()[boundary_index[i]].value.lang_type() == CompileVocab::KEYWORD(LBKeyword::Func) {
            for token in flatmap_node_tokens(&mut fabric.tokens()[boundary_index[i]..boundary_index[i + 1]].to_vec()) { mini_stack.insert(mini_stack.len(), token) }
        } else {
            create_stack(&mut fabric.tokens()[boundary_index[i]..boundary_index[i + 1]].to_vec(), &mut mini_stack);
            mini_stack.reverse();
        }
        if stack_print {
            for token in &mini_stack {
                if token.lang_type().matches("boundary") { /*println!("{:?}", token);*/ } else {
                   println!("{:?}", token);
                }
            }
        }
        stack.append(&mut mini_stack);
    }

    stack
}