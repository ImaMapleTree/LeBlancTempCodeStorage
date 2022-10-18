use std::ops::Deref;
use std::sync::Arc;
use arrayvec::ArrayVec;
use crate::leblanc::core::interpreter::execution_context::ExecutionContext;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::{get_globals, get_handles};
use crate::leblanc::core::leblanc_object::{LeBlancObject, Stringify};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::blueberry::{BlueberryPush, BlueberryVec, Quantum};
use crate::leblanc::rustblanc::types::{IExec, IExecResult, LBObject, LeBlancStack};

#[inline]
fn safe_stack_pop(stack: &mut BlueberryVec<LeBlancObject>) -> Result<LBObject, LBObject> {
    //println!("My stack: {:?}", stack);
    match stack.pop() {
        None => {
            println!("Hit &mut context.stack error");
            Err(LeblancError::new("Unknow &mut context.stackException".to_string(), "Internal &mut context.stack pop returned a none value".to_string(), vec![]).create_mutex())
        }

        Some(result) => {
            Ok(result)
        }
    }
}

#[inline]
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
        Instruction2::RETURN(_, _) => iexec_return,
        _ => iexec_no_ref
    }
}

#[inline]
fn iexec_no_ref(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    println!("I don't exist :)");
    Err(LeblancError::new("Instruction Doesn't Exist".to_string(), "".to_string(), vec![]).create_mutex())
}

#[inline]
fn iexec_badd_native(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let targeter = safe_stack_pop(&mut context.handle_ref.stack)?;
    let binding1 = targeter.reference();
    let target = safe_stack_pop(&mut context.handle_ref.stack)?;
    let binding2 = target.reference();
    //println!("Binding 1: {:?} || Binding 2: {:?}", binding1.data, binding2.data);
    //let addition =

    context.handle_ref.stack.push(leblanc_object_int((binding2.data.as_i64() + binding1.data.as_i64()) as i32));

    Ok(())
}

#[inline]
fn iexec_bsub_native(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let targeter = safe_stack_pop(&mut context.handle_ref.stack)?;
    let target = safe_stack_pop(&mut context.handle_ref.stack)?;
    //println!("Binding 1: {:?} || Binding 2: {:?}", targeter.reference().data, target.reference().data);
    //println!("TARGET: {:?}", target);
    //println!("TARGETER: {:?}", targeter);
    let result = leblanc_object_int((target.reference().data.as_i64() - targeter.reference().data.as_i64()) as i32);
    //println!("RESULT: {:?}", result); 

    context.handle_ref.stack.push(result);
    Ok(())
}


#[inline]
fn iexec_load_const(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let result= context.get_constant(instruct.bytes()[0] as usize);
    if result.is_none() { return Err(LeBlancObject::error().to_mutex()) };
    let pointer = result.unwrap().pointer();
    context.handle_ref.stack.push(pointer);
    Ok(())
}

#[inline]
fn iexec_load_var(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes();
    let result= context.variables.get(bytes[0] as usize);
    if let Some(lbo) = result {
        context.handle_ref.stack.push(lbo);
    } else {
       context.handle_ref.variables.get(bytes[0] as usize).expect("Death");
    }
    Ok(())
}

#[inline]
fn iexec_store_var(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    context.variables.set_smart(instruct.bytes()[0] as usize, safe_stack_pop(&mut context.handle_ref.stack)?).expect("Something went really wrong");
    Ok(())
}

#[inline]
fn iexec_builtin(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes();
    let func= unsafe { get_globals() }[bytes[0] as usize].clone();

    let mut arguments = vec![safe_stack_pop(&mut context.handle_ref.stack)?; bytes[1] as usize];
    arguments.reverse();


    //println!("Arguments: {:#?}", arguments);
    let handle = func.reference().data.get_inner_method().unwrap().handle;
    let result = handle(func, arguments);

    let typing = result.reference().typing;
    match typing {
        LeBlancType::Exception => return Err(result),
        _ => { context.handle_ref.stack.push(result.to_owned()); }
    }
    Ok(())
}


#[inline]
fn iexec_call_normal(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes();
    let handle= &mut get_handles()[bytes[0] as usize];
    let mut arguments = vec![safe_stack_pop(&mut context.handle_ref.stack)?; bytes[1] as usize];
    arguments.reverse();

    let mut deferred = handle.defer(arguments);
    let result = deferred.execute();

    context.handle_ref.stack.push(result.to_owned());
    Ok(())
}

#[inline]
fn iexec_if_le(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = safe_stack_pop(&mut context.handle_ref.stack)?;
    let s2 = safe_stack_pop(&mut context.handle_ref.stack)?;
    //println!("{:?} >= {:?}", s1.lock().data, s2.lock().data);
    if s1.reference().data < s2.reference().data {
        context.instruction_pointer += instruct.bytes()[0] as usize;
    }
    Ok(())
}


#[inline]
fn iexec_return(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    //println!("Storage: {:?}", context.variables);
    Err(Quantum::default())
}
