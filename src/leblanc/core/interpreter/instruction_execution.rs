use alloc::rc::Rc;
use alloc::sync;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, Weak};
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_handler::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_tag::MethodTag;

pub fn execute_instruction(instruct: InstructionBase) -> fn(&mut LeblancHandle, u16, &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    return match instruct {
        InstructionBase::BinaryAdd => _INSTRUCT_BINARY_ADD_,
        InstructionBase::InPlaceAdd => _INSTRUCT_INPLACE_ADD_,
        InstructionBase::LoadLocal => _INSTRUCT_LOAD_LOCAL_,
        _ => _INSTRUCT_BASE_
    }
}

fn safe_stack_pop(stack: &mut Vec<Arc<Mutex<LeBlancObject>>>, mut error: bool) -> Arc<Mutex<LeBlancObject>> {
    stack.pop().unwrap_or_else(|| {error = true; Arc::new(Mutex::new(LeBlancObject::error())) })
}

fn _INSTRUCT_BASE_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    Err(Arc::new(Mutex::new(LeBlancObject::error())))
}

fn _INSTRUCT_INPLACE_ADD_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut error = false;
    let mut target = safe_stack_pop(stack, error);
    if error { return Err(target); }
    let mut arg1 =  safe_stack_pop(stack, error);
    if error { return Err(arg1); }


    println!("testing result");
    let mut result = target.lock().unwrap().methods.iter().cloned().filter(|m| m.has_tag(MethodTag::InPlaceAddition)).next();
        //.filter(|m| m.matches("_".to_string(), vec![tos2.lock().unwrap().to_leblanc_arg(0)]))
        //.next().unwrap_or(Method::error()).run(tos1.clone(), &mut [tos2.clone()]);

    println!("result: {:#?}", result);
    println!("target: {:?}", target.lock().unwrap().data);
    println!("arg1: {:?}", arg1.lock().unwrap().data);
    result.unwrap().run(target.clone(), &mut [arg1.clone()]);
    println!("haha I ran");
    let result = Arc::new(Mutex::new(LeBlancObject::null()));
    println!("testing result");

    println!("testing result");
    stack.push(result);
    Ok(())
}

fn _INSTRUCT_BINARY_ADD_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut error = false;
    let mut tos1 =  safe_stack_pop(stack, error);
    if error { return Err(tos1); }
    let mut tos2 = safe_stack_pop(stack, error);
    if error { return Err(tos2); }

    let mut result = tos1.lock().unwrap().methods.iter().filter(|m| m.has_tag(MethodTag::Addition)).filter(|m| m.matches("_".to_string(), vec![tos2.lock().unwrap().to_leblanc_arg(0)]))
        .next().unwrap_or(&Method::error()).run(tos1.clone(), &mut [tos2.clone()]);

    if result.lock().unwrap().is_error() {
        tos2.lock().unwrap().methods.iter().filter(|m| m.has_tag(MethodTag::Addition)).filter(|m| m.matches("_".to_string(), vec![tos1.lock().unwrap().to_leblanc_arg(0)]))
            .next().unwrap_or(&Method::error()).run(tos2.clone(), &mut [tos1.clone()]);
    }

    stack.push(result);
    Ok(())
}


fn _INSTRUCT_LOAD_FUNCTION_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= handle.globals.get(0);
    if result.is_none() { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    stack.push(result.unwrap().clone());
    Ok(())
}

fn _INSTRUCT_LOAD_CONSTANT_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= handle.globals.get(0);
    if result.is_none() { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    stack.push(result.unwrap().clone());
    Ok(())
}

fn _INSTRUCT_LOAD_LOCAL_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    println!("Load local :)");
    let result= handle.variables.get(arg as usize);
    println!("variables: {:#?}", handle.variables);
    if result.is_none() { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    stack.push(result.unwrap().clone());
    Ok(())
}

fn _CALL_FUNCTION_(handle: &mut LeblancHandle, arg: u16, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut error = false;
    let mut arguments = vec![];
    let mut func = safe_stack_pop(stack, error);
    if error { return Err(func); }
    for _ in 0..arg {
        let tos = safe_stack_pop(stack, error);
        if error { return Err(tos); }
        arguments.insert(0, tos);
    }

    let result = func.call("execute", &mut arguments);
    stack.push(result);
    Ok(())
}