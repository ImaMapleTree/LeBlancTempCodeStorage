
use std::collections::BTreeSet;
use std::io;



use alloc::rc::Rc;
use std::cell::RefCell;

use prettytable::{Cell, format, Row, Table};
use crate::leblanc::core::interpreter::instructions::InstructionBase;
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{LeBlancObject, Reflect, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;


use crate::LeBlancType;

static mut STDOUT: Option<io::Stdout> = None;

fn _BUILTIN_DISASSEMBLE(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let method = args[0].borrow().data.get_inner_method().unwrap().clone();
    let dis_rust_func = if args.len() > 1 {
        *args[1].reflect().downcast_ref::<bool>().unwrap()
    } else {
        false
    };

    let mut output = String::new();
    if method.is_internal_method() {
        if dis_rust_func {
            println!("Disassembling builtin");
        }
    } else {
        let leblanc_handle = method.leblanc_handle;
        let instructions = leblanc_handle.borrow().instructions.clone();
        let mut prev_line_number = 0;
        let mut line_number_format = grow_to_size("", 8);
        let mut instruct_count = 0;
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        for instruction in instructions.iter() {
            if instruction.line_number != prev_line_number {
                line_number_format = grow_to_size(&instruction.line_number.to_string(), 8);
                prev_line_number = instruction.line_number;
                table.add_row(Row::new(vec![Cell::new("").with_hspan(5)]));
            } else {line_number_format = grow_to_size("", 8)}

            let arg_string = match instruction.instruct {
                InstructionBase::LoadLocal => format!("({})", leblanc_handle.borrow().variable_context.values().find(|context| context.relationship == instruction.arg as u32).unwrap().name),
                InstructionBase::LoadConstant => format!("({})", leblanc_handle.borrow().constants[instruction.arg as usize].borrow().data),
                InstructionBase::LoadFunction => format!("({})", unsafe {get_globals()[instruction.arg as usize].borrow().data.get_inner_method().unwrap().context.name.clone()}),
                InstructionBase::Equality(_) => format!("({})", recover_equality_op(instruction.arg as u8)),
                _ => "".to_string()
            };
            table.add_row(Row::new(vec![
                Cell::new(&line_number_format),
                Cell::new(&instruct_count.to_string()),
                Cell::new(&(instruction.instruct.to_string())),
                Cell::new(&instruction.arg.to_string()),
                Cell::new(&arg_string)
            ]));
            instruct_count += 2;
        }
        output = table.to_string();
    }

    unsafe {
        if STDOUT.is_none() {
            STDOUT = Some(io::stdout());
        }
        io::copy(&mut output.as_bytes(), &mut STDOUT.as_mut().unwrap()).unwrap();
    }
    LeBlancObject::unsafe_null()
}

pub fn _BUILTIN_DISASSEMBLE_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "dis".to_string(),
            vec![LeBlancArgument::default(LeBlancType::Function, 0), LeBlancArgument::optional(LeBlancType::Boolean, 1)]
        ),
        _BUILTIN_DISASSEMBLE,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_DISASSEMBLE_OBJECT_() -> LeBlancObject {
    internal_method(_BUILTIN_DISASSEMBLE_METHOD_())
}

fn grow_to_size(string: &str, number: usize) -> String {
    let mut new_string = string.to_string();
    while new_string.len() < number {
        new_string += " ";
    }
    new_string
}

fn recover_equality_op(n: u8) -> String {
    match n {
        0 => String::from("=="),
        1 => String::from("!="),
        2 => String::from(">"),
        3 => String::from("<"),
        4 => String::from(">="),
        5 => String::from("<="),
        _ => String::from("")
    }
}