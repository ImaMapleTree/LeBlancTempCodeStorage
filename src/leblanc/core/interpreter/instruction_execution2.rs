use std::sync::Arc;
use arrayvec::ArrayVec;
use crate::leblanc::core::interpreter::execution_context::ExecutionContext;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::leblanc_object::{LeBlancObject, QuickUnwrap, Stringify};
use crate::leblanc::core::native_types::attributes::can_add_self;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::rustblanc::types::{IExec, IExecResult, LBObject, LeBlan &mut context.stack};

fn safe_stack_pop &mut context.stack: &mut LeBlan &mut context.stack) -> Result<LBObject, LBObject> {
    match &mut context.stack.pop() {
        None => {
            println!("Hit &mut context.stack error");
            Err(LeblancError::new("Unknow &mut context.stackException".to_string(), "Internal &mut context.stack pop returned a none value".to_string(), vec![]).create_mutex())
        }
        Some(result) => Ok(result)
    }
}

pub fn execute_instruction2(instruct: Instruction2) -> IExec {
    match instruct {
        Instruction2::NOREF(_, _) => iexec_no_ref,
        Instruction2::BADD_NATIVE(_, _) => iexec_badd_native,
        Instruction2::BSUB_NATIVE(_, _) => iexec_bsub_native,
        Instruction2::LOAD_CONSTANT(_, _) => iexec_load_const,
        Instruction2::LOAD_VARIABLE(_, _) => iexec_load_var,
        Instruction2::STORE_VARIABLE(_, _) => iexec_store_var,
        Instruction2::STORE_CINV(_, _) => iexec_no_ref,
        Instruction2::CALL_BUILTIN(_, _) => iexec_builtin,
        Instruction2::CALL_NORMAL(_, _) => iexec_call_normal,
        Instruction2::IF_LESS_EQUALS(_, _) => iexec_if_le,
        Instruction2::RETURN(_, _) => iexec_no_ref,
        _ => iexec_no_ref
    }
}

fn iexec_no_ref(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    println!("I don't exist :)");
    Err(LeblancError::new("Instruction Doesn't Exist".to_string(), "".to_string(), vec![]).create_mutex())
}

fn iexec_badd_native(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let targeter = safe_stack_pop(&mut context.stack)?;
    let target = safe_stack_pop(&mut context.stack)?; 

   &mut context.stack.push(leblanc_object_int((target.underlying_pointer().data.as_i64() + targeter.underlying_pointer().data.as_i64()) as i32).to_mutex());
    Ok(())
}

fn iexec_bsub_native(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let targeter = safe_stack_pop(&mut context.stack)?;
    let target = safe_stack_pop(&mut context.stack)?;
    //println!("TARGET: {:?}", target);
    //println!("TARGETER: {:?}", targeter);
    let result = leblanc_object_int((target.underlying_pointer().data.as_i64() - targeter.underlying_pointer().data.as_i64()) as i32).to_mutex();
    //println!("RESULT: {:?}", result); 

   &mut context.stack.push(result);
    Ok(())
}


fn iexec_load_const(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let result= context.get_constant(instruct.bytes()[0] as usize);
    if let Some(constant) = result { 
       &mut context.stack.push(constant.clone());
        return Ok(())
    }
    Err(LeBlancObject::error().to_mutex())
}

fn iexec_load_var(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes();
    let result= context.variables.get(bytes[0] as usize);
    if let Some(lbo) = result { 
       &mut context.stack.push(lbo.clone());
    } else { 
       &mut context.stack.push(LeBlancObject::null().to_mutex());
    }
    Ok(())
}

fn iexec_store_var(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    handle.variables[instruct.bytes()[0] as usize] = safe_stack_pop &mut context.stack)?; Ok(())
}

fn iexec_builtin(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes();
    let func= unsafe { get_globals() }[bytes[0] as usize].clone();
    let mut arguments = vec![LeBlancObject::unsafe_null(); bytes[1] as usize];
    for arg in arguments.iter_mut().rev() {
        *arg = safe_stack_pop(&mut context.stack)?;
    }

    let handle = func.underlying_pointer().data.get_inner_method().unwrap().handle;
    let result = handle(func, &mut arguments);

    let typing = result.underlying_pointer().typing;
    match typing {
        LeBlancType::Exception => return Err(result),
        _ => &mut context.stack.push(result)
    }
    Ok(())
}

fn iexec_call_normal(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes();
    let func= &mut unsafe { get_globals() }[bytes[0] as usize];
    let mut arguments = vec![LeBlancObject::unsafe_null(); bytes[1] as usize];
    for arg in arguments.iter_mut().rev() {
        *arg = safe_stack_pop(&mut context.stack)?;
    }

    //let func = func.clone_if_locked();
    let result = func.underlying_pointer().data.get_inner_method().unwrap().leblanc_handle.clone_if_locked().lock().execute(&mut arguments);

    if result.underlying_pointer().typing == LeBlancType::Exception {
         Err(result)
    } else { Ok &mut context.stack.push(result)) }
}

fn iexec_if_le(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = safe_stack_pop(&mut context.stack)?;
    let s2 = safe_stack_pop(&mut context.stack)?;
    //println!("{:?} >= {:?}", s1.lock().data, s2.lock().data);
    if s1.lock().data < s2.lock().data {
        context.instruction_pointer += instruct.bytes()[0] as usize;
    }
    Ok(())
}