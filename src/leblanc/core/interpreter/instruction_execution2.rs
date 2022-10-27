
use crate::leblanc::core::interpreter::execution_context::ExecutionContext;
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::leblanc_runner::{get_globals, get_handles};
use crate::leblanc::core::leblanc_object::{LeBlancObject};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::{IExec, IExecResult, LBObject};
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;

#[inline(always)]
fn safe_stack_pop(stack: &mut Vec<LBObject>) -> Result<LBObject, LBObject> {
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

#[inline(always)]
pub fn execute_instruction2(instruct: Instruction2) -> IExec {
    match instruct {
        Instruction2::NOREF(_, _) => iexec_no_ref,
        Instruction2::BADD_NATIVE(_, _) => iexec_badd_native,
        Instruction2::BSUB_NATIVE(_, _) => iexec_bsub_native,
        Instruction2::LOAD_CONSTANT(_, _) => iexec_load_const,
        Instruction2::LOAD_VARIABLE(_, _) => iexec_load_var,
        Instruction2::STORE_VARIABLE(_, _) => iexec_store_var,
        Instruction2::STORE_CINV(_, _) => iexec_no_ref,
        Instruction2::LOAD_FUNCTION(_, _) => iexec_no_ref,
        Instruction2::CALL_BUILTIN(_, _) => iexec_builtin,
        Instruction2::CALL_NORMAL(_, _) => iexec_call_normal,
        Instruction2::IF_EQUALS(_, _) => iexec_if_eq,
        Instruction2::IF_NOT_EQUALS(_, _) => iexec_if_ne,
        Instruction2::IF_GREATER_EQUALS(_, _) => iexec_if_ge,
        Instruction2::IF_GREATER(_, _) => iexec_if_gt,
        Instruction2::IF_LESS_EQUALS(_, _) => iexec_if_le,
        Instruction2::IF_LESS(_, _) => iexec_if_lt,
        Instruction2::JUMP(_, _) => iexec_jump,
        Instruction2::JUMP_BACK(_, _) => iexec_jump_back,
        Instruction2::RETURN(_, _) => iexec_return,
        _ => iexec_no_ref
    }
}

#[inline(always)]
fn iexec_no_ref(_context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    println!("I don't exist :): {:?}", instruct);
    Err(LeblancError::new("Instruction Doesn't Exist".to_string(), "".to_string(), vec![]).create_mutex())
}

#[inline(always)]
fn iexec_badd_native(context: &mut ExecutionContext, _instruct: Instruction2) -> IExecResult {
    let targeter = unsafe { context.handle_ref.stack.pop_unsafe() };
    let target = unsafe { context.handle_ref.stack.pop_unsafe() };
    unsafe { context.handle_ref.stack.push_quick(leblanc_object_int((targeter.data.as_i64() + target.data.as_i64()) as i32)); }
    Ok(())
}

#[inline(always)]
fn iexec_bsub_native(context: &mut ExecutionContext, _instruct: Instruction2) -> IExecResult {
    let targeter = unsafe { context.handle_ref.stack.pop_unsafe() };
    let target = unsafe { context.handle_ref.stack.pop_unsafe() };
    unsafe { context.handle_ref.stack.push_quick(leblanc_object_int((target.data.as_i64() - targeter.data.as_i64()) as i32)); }
    Ok(())
}


#[inline(always)]
fn iexec_load_const(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let constant = unsafe {context.handle_ref.constants.get_unchecked(instruct.byte(0) as usize)}.clone();
    //println!("constant: {:?}", constant);
    //println!("Handle: {:?}", context.handle_ref);
    unsafe { context.handle_ref.stack.push_quick(constant); }
    Ok(())
}

#[inline(always)]
fn iexec_load_var(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let result= unsafe {context.variables.get_unsafe(instruct.byte(0) as usize)}.clone();
    //println!("var: {:?}", result);
    unsafe { context.handle_ref.stack.push_quick(result); }
    Ok(())
}

#[inline(always)]
fn iexec_store_var(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    unsafe { context.variables.set_unchecked(instruct.byte(0) as usize,context.handle_ref.stack.pop_unsafe()) }
    Ok(())
}

/*#[inline(always)]
fn iexec_load_func(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let func= unsafe { get_globals().get_unsafe(instruct.byte(0) as usize)}.clone();
    unsafe { context.handle_ref.stack.push_quick(func); }
    Ok(())
}*/

#[inline(always)]
fn iexec_builtin(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes2(2);
    let func= unsafe { get_globals().get_unsafe(instruct.byte(0) as usize)}.clone();
    //println!("Func: {:?}", func);
    //let mut arguments = vec![safe_stack_pop(&mut context.handle_ref.stack)?; bytes[1] as usize];
    //arguments.reverse();
    let stack = &mut context.handle_ref.stack;
    let arguments = unsafe { stack.split_off_bounded(stack.len() - bytes[1] as usize) };

    //println!("Arguments: {:#?}", arguments);
    let handle = func.handle;
    let result = handle(LeBlancObject::null(), arguments);

    let typing = result.typing;
    match typing {
        16 => return Err(result),
        _ => unsafe { context.handle_ref.stack.push_quick(result); }
    }
    Ok(())
}


#[inline(always)]
fn iexec_call_normal(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let bytes = instruct.bytes2(2);
    let handle= unsafe { &mut get_handles().get_unsafe(bytes[0] as usize) };
    //println!("handle: {:?}", handle.constants);
    let stack = &mut context.handle_ref.stack;
    //println!("own context: {:?}", handle.constants);
    let arguments = unsafe { stack.split_off_bounded(stack.len() - bytes[1] as usize) };
    //println!("Arguments: {:?}", arguments);

    let result = handle.execute(arguments);

    //println!("handle: {:?}", handle.constants);

    unsafe { context.handle_ref.stack.push_quick(result); }
    Ok(())
}

#[inline(always)]
fn iexec_if_eq(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = unsafe { context.handle_ref.stack.pop_unsafe() };
    let s2 = unsafe { context.handle_ref.stack.pop_unsafe() };
    //println!("{:?} >= {:?}", s1.lock().data, s2.lock().data);
    if s1.data != s2.data {
        context.instruction_pointer += instruct.byte(0) as usize;
    }
    Ok(())
}

#[inline(always)]
fn iexec_if_ne(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = unsafe { context.handle_ref.stack.pop_unsafe() };
    let s2 = unsafe { context.handle_ref.stack.pop_unsafe() };
    //println!("{:?} >= {:?}", s1.lock().data, s2.lock().data);
    if s1.data == s2.data {
        context.instruction_pointer += instruct.byte(0) as usize;
    }
    Ok(())
}

#[inline(always)]
fn iexec_if_le(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = unsafe { context.handle_ref.stack.pop_unsafe() };
    let s2 = unsafe { context.handle_ref.stack.pop_unsafe() };
    //println!("{:?} >= {:?}", s1.lock().data, s2.lock().data);
    if s1.data < s2.data {
        context.instruction_pointer += instruct.byte(0) as usize;
    }
    Ok(())
}

#[inline(always)]
fn iexec_if_lt(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = unsafe { context.handle_ref.stack.pop_unsafe() };
    let s2 = unsafe { context.handle_ref.stack.pop_unsafe() };
    //println!("{:?} >= {:?}", s1.data, s2.data);
    if s1.data <= s2.data {
        context.instruction_pointer += instruct.byte(0) as usize;
    }
    Ok(())
}

#[inline(always)]
fn iexec_if_ge(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = unsafe { context.handle_ref.stack.pop_unsafe() };
    let s2 = unsafe { context.handle_ref.stack.pop_unsafe() };
    //println!("{:?} >= {:?}", s1.lock().data, s2.lock().data);
    if s1.data > s2.data {
        context.instruction_pointer += instruct.byte(0) as usize;
    }
    Ok(())
}

#[inline(always)]
fn iexec_if_gt(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    let s1 = unsafe { context.handle_ref.stack.pop_unsafe() };
    let s2 = unsafe { context.handle_ref.stack.pop_unsafe() };
    if s1.data >= s2.data {
        context.instruction_pointer += instruct.byte(0) as usize;
    }
    Ok(())
}

#[inline(always)]
fn iexec_jump(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    context.instruction_pointer += instruct.byte(0) as usize;
    Ok(())
}

#[inline(always)]
fn iexec_jump_back(context: &mut ExecutionContext, instruct: Instruction2) -> IExecResult {
    // We need to add 1 because our instruction counter is going to increase after this is run
    context.instruction_pointer -= (1 + instruct.byte(0) as usize);
    Ok(())
}


#[inline(always)]
fn iexec_return(_context: &mut ExecutionContext, _instruct: Instruction2) -> IExecResult {
    _context.should_return = true;
    Ok(())
}
