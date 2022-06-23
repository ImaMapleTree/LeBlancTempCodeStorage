use core::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::instructions::InstructionBase::{Comparator_Else, Comparator_ElseIf, Comparator_If};
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_object::{ArcType, Callable, LeBlancObject, LeBlancObjectData, Reflect};
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::attributes::can_add_self;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::derived::DerivedType::List;
use crate::leblanc::core::native_types::derived::list_type::{leblanc_object_list_empty, LeblancList};
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::rustblanc::utils::Timing;
use crate::LeBlancType;
use crate::leblanc::core::leblanc_handle::execute_handle;
use crate::leblanc::rustblanc::strawberry::Either;

pub fn execute_instruction(instruct: InstructionBase) -> fn(&mut LeblancHandle, &Instruction, &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    return match instruct {
        InstructionBase::BinaryAdd => _INSTRUCT_BINARY_ADD_,
        InstructionBase::BinarySubtract => _INSTRUCT_BINARY_SUBTRACT_,
        InstructionBase::InPlaceAdd => _INSTRUCT_INPLACE_ADD_,
        InstructionBase::LoadLocal => _INSTRUCT_LOAD_LOCAL_,
        InstructionBase::LoadConstant => _INSTRUCT_LOAD_CONSTANT_,
        InstructionBase::LoadFunction => _INSTRUCT_LOAD_FUNCTION_,
        InstructionBase::StoreLocal => _INSTRUCT_STORE_LOCAL_,
        InstructionBase::CallFunction => _CALL_FUNCTION_,
        InstructionBase::CallClassMethod => _INSTRUCT_CALL_CLASS_METHOD_,
        InstructionBase::QuickList(_) => _INSTRUCT_CREATE_RANGE_,
        InstructionBase::ForLoop => _INSTRUCT_FOR_LOOP_,
        InstructionBase::Equality(_) => _INSTRUCT_EQUALITY_,
        Comparator_If => _INSTRUCT_COMPARATOR_,
        Comparator_ElseIf => _INSTRUCT_COMPARATOR_,
        Comparator_Else => _INSTRUCT_COMPARATOR_,
        _ => _INSTRUCT_BASE_
    }
}


fn deprecated_safe_stack_pop(stack: &mut Vec<Arc<Mutex<LeBlancObject>>>, mut error: bool) -> Arc<Mutex<LeBlancObject>> {
    return match stack.pop() {
        None => {
            error = true;
            LeBlancObject::unsafe_error()
        }
        Some(result) => result
    };
}

fn safe_stack_pop(stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<Arc<Mutex<LeBlancObject>>, Arc<Mutex<LeBlancObject>>> {
    return match stack.pop() {
        None => {
            Err(LeBlancObject::unsafe_error())
        }
        Some(result) => Ok(result)
    };
}

fn _INSTRUCT_BASE_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    Err(Arc::new(Mutex::new(LeBlancObject::error())))
}

fn _INSTRUCT_INPLACE_ADD_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let target = deprecated_safe_stack_pop(stack, error);
    if error { return Err(target); }
    let arg1 =  deprecated_safe_stack_pop(stack, error);
    if error { return Err(arg1); }


    let result = target.lock().unwrap().methods.iter().cloned().filter(|m| m.has_tag(MethodTag::InPlaceAddition)).next();
        //.filter(|m| m.matches("_".to_string(), vec![tos2.lock().unwrap().to_leblanc_arg(0)]))
        //.next().unwrap_or(Method::error()).run(tos1.clone(), &mut [tos2.clone()]);

    result.unwrap().run(target.clone(), &mut [arg1.clone()]);
    let result = Arc::new(Mutex::new(LeBlancObject::null()));
    stack.push(result);
    Ok(())
}

fn _INSTRUCT_BINARY_ADD_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let mut targeter_unwrap = targeter.lock().unwrap();
    if Mutex::try_lock(&target).is_err() {
        target = Arc::new(Mutex::new(targeter_unwrap._clone()));
    }
    let mut target_unwarp = target.lock().unwrap();
    if can_add_self(&targeter_unwrap.typing) && can_add_self(&target_unwarp.typing) {
        stack.push((target_unwarp.data.as_i128() + targeter_unwrap.data.as_i128()).create_mutex());
        return Ok(());
    }

    let arguments = vec![targeter.lock().unwrap().to_leblanc_arg(0)];
    let target_clone = Arc::clone(&target);
    let matched_method = target_clone.lock().unwrap().methods.iter().filter(|m| {
        m.matches("_".to_string(), &arguments)
    }).next().cloned();
    match matched_method {
        None => {
            let arguments = vec![target.lock().unwrap().to_leblanc_arg(0)];
            let targeter_clone = Arc::clone(&targeter);
            let matched_method = targeter_clone.lock().unwrap().methods.iter().filter(|m| {
                m.matches("_".to_string(), &arguments)
            }).next().cloned();
            if matched_method.is_none() {
                return Err(Arc::new(Mutex::new(LeBlancObject::error())));
            }
            stack.push(matched_method.unwrap().run(targeter_clone, &mut [target_clone]));
        }
        Some(mut method) => {
            stack.push(method.run(target_clone, &mut [targeter.clone()]));
        }
    }

    Ok(())
}

fn _INSTRUCT_BINARY_SUBTRACT_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut target =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let targeter = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let mut targeter_unwrap = targeter.lock().unwrap();
    if Mutex::try_lock(&target).is_err() {
        target = targeter_unwrap._clone().to_mutex()
    }
    let mut target_unwarp = target.lock().unwrap();

    if can_add_self(&targeter_unwrap.typing) && can_add_self(&target_unwarp.typing) {
        stack.push((target_unwarp.data.as_i128() - targeter_unwrap.data.as_i128()).create_mutex());
        return Ok(());
    }

    let arguments = vec![targeter.lock().unwrap().to_leblanc_arg(0)];
    let target_clone = Arc::clone(&target);
    let matched_method = target_clone.lock().unwrap().methods.iter().filter(|m| {
        m.matches("_".to_string(), &arguments)
    }).next().cloned();
    match matched_method {
        None => {
            let arguments = vec![target.lock().unwrap().to_leblanc_arg(0)];
            let targeter_clone = Arc::clone(&targeter);
            let matched_method = targeter_clone.lock().unwrap().methods.iter().filter(|m| {
                m.matches("_".to_string(), &arguments)
            }).next().cloned();
            if matched_method.is_none() {
                return Err(LeBlancObject::error().to_mutex());
            }
            stack.push(matched_method.unwrap().run(targeter_clone, &mut [target_clone]));
        }
        Some(mut method) => {
            stack.push(method.run(target_clone, &mut [targeter.clone()]));
        }
    }

    Ok(())
}


#[inline(always)]
fn _INSTRUCT_LOAD_FUNCTION_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= unsafe { get_globals() }.get(arg.arg as usize);
    if result.is_none() { LeBlancObject::error().to_mutex(); }
    stack.push(result.unwrap().clone());
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_LOAD_CONSTANT_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= handle.constants.get(arg.arg as usize);
    match result {
        None => return Err(LeBlancObject::error().to_mutex()),
        Some(constant) => {
            stack.push(constant._clone().to_mutex());
            Ok(())
        }
    }

}

#[inline(always)]
fn _INSTRUCT_LOAD_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= handle.variables.get(arg.arg as usize).cloned();
    match result {
        None => {
            let null = LeBlancObject::null().to_mutex();
            if arg.arg as usize >= handle.variables.len() {
                handle.variables.push(null.clone());
            } else {
                handle.variables[arg.arg as usize] = null.clone();
            }
            stack.push(null);
        },
        Some(res) => {
            match res.leblanc_type().is_numeric() {
                true => stack.push(res.clone()),
                false => stack.push(res.clone())
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_STORE_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    if arg.arg as usize >= handle.variables.len() {
        handle.variables.push(result);
    } else {
        handle.variables[arg.arg as usize] = result;
    }
    Ok(())
}

fn _CALL_FUNCTION_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let func = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut arguments = match arg.arg {
        0 => { Vec::new() }
        1 => { vec![match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) }] }
        _ => {
            let mut vector = Vec::new();
            for _ in 0..arg.arg as usize {
                let tos = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
                vector.push(tos);
            }
            vector.reverse();
            vector
        }
    };
    let mut mutex = func.lock().unwrap();
    let mutex_method = mutex.data.retrieve_self_as_function().unwrap();
    let handle = mutex_method.handle;
    let is_internal = mutex_method.is_internal_method();
    let mut leblanc_handle = mutex_method.leblanc_handle.clone();


    Mutex::unlock(mutex);

    stack.push( match is_internal {
        true => (handle)(func.clone(), &mut arguments),
        false => leblanc_handle.loan().inquire().either().execute(&mut arguments)
    });

    /*let result = match acquire.acquire() {
        Ok(lock) => {
            null_handle = lock.get().lock().unwrap().null;
            if null_handle { LeBlancObject::unsafe_null() }
            else { lock.get().lock().as_mut().unwrap().execute(&mut arguments) }
        },
        Err(mut lock) => {
            null_handle = unsafe {&mut **lock.get().lock().unwrap()}.null;
            if null_handle { LeBlancObject::unsafe_null() }
            else { unsafe {&mut **lock.get().lock().unwrap()}.execute(&mut arguments) }
        },
    };
    //let null_handle = leblanc_handle.lock().unwrap().null;
    let result = match null_handle {
        false => { result }
        true => {
            (func.lock().unwrap().data.retrieve_self_as_function().unwrap().handle)(func.clone(), &mut arguments)
        }
    };*/

    //stack.push(result);

    Ok(())
}

fn _INSTRUCT_CALL_CLASS_METHOD_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let method_name = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut arguments = match arg.arg {
        0 => { Vec::new() }
        1 => { vec![deprecated_safe_stack_pop(stack, error)] }
        _ => {
            let mut vector = Vec::new();
            for _ in 0..arg.arg as usize {
                let tos = deprecated_safe_stack_pop(stack, error);
                if error { return Err(tos); }
                vector.push(tos);
            }
            vector.reverse();
            vector
        }
    };
    let mut object = deprecated_safe_stack_pop(stack, error);
    if error { return Err(object); }
    stack.push(object.call(method_name.lock().unwrap().data.to_string().as_str(), &mut arguments));
    Ok(())
}

fn _INSTRUCT_CREATE_RANGE_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut increment = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut operand = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let bound = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let mut internal_list = LeblancList::empty();

    /*println!("Operand: {:?}", operand);
    println!("Bound: {:?}", bound);
    println!("Increment: {:?}", increment);*/
    while operand.lock().unwrap().data < bound.lock().unwrap().data || (*&increment.leblanc_type() == LeBlancType::Boolean && *increment.reflect().downcast_ref::<bool>().unwrap()){
        //println!("{}", internal_list);
        internal_list.internal_vec.push(operand.lock().unwrap()._clone().to_mutex());
        if increment.leblanc_type() == LeBlancType::Function {
            operand = increment.lock().unwrap().data.retrieve_self_as_function().unwrap().run(increment.clone(), &mut [operand]).lock().unwrap()._clone().cast(bound.leblanc_type()).to_mutex();
        } else if operand.leblanc_type().is_native() {
            operand = operand.call("_ADD_", &mut [increment.clone()]).lock().unwrap()._clone().cast(bound.leblanc_type()).to_mutex();
        } else {
            let matched_method = increment.lock().unwrap().methods.iter().filter(|m| {
                m.matches("_".to_string(), &vec![operand.lock().unwrap().to_leblanc_arg(0)])
            }).next().cloned();
            match matched_method {
                None => {return Err(increment)}
                Some(mut method) => {operand = method.run(increment.clone(), &mut [operand])}}
        }
        //println!("Operand: {:?}", operand);
    }

    let mut arc = internal_list.create_mutex();
    stack.push(arc);

    Ok(())
}

fn _INSTRUCT_FOR_LOOP_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut iter_variable = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let iterable = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let loop_start = handle.current_instruct;

    // make into an iterator eventually, right now we'll just grab the internal list
    for item in &iterable.reflect().downcast_ref::<LeblancList>().unwrap().internal_vec {
        iter_variable.lock().unwrap().copy_data(&item.lock().unwrap());
        let loop_result = handle.execute_range(loop_start+1, loop_start+1 + arg.arg as u64 );
    }

    //println!("Done with loop");
    handle.current_instruct = loop_start;
    handle.current_instruct += arg.arg as u64;

    Ok(())
}

fn _INSTRUCT_EQUALITY_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let tos1 = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let tos2 = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let tos1 = tos1.lock().unwrap().cast(tos2.leblanc_type()).to_mutex();

    /*println!("TOS1: {:?}", tos1.lock().unwrap().data);
    println!("TOS2: {:?}", tos2.lock().unwrap().data);*/

    let result = match arg.arg {
        0 => (tos1.lock().unwrap().data == tos2.lock().unwrap().data).create_mutex(),
        1 => (tos1.lock().unwrap().data != tos2.lock().unwrap().data).create_mutex(),
        2 => (tos1.lock().unwrap().data > tos2.lock().unwrap().data).create_mutex(),
        3 => (tos1.lock().unwrap().data < tos2.lock().unwrap().data).create_mutex(),
        4 => (tos1.lock().unwrap().data >= tos2.lock().unwrap().data).create_mutex(),
        5 => (tos1.lock().unwrap().data <= tos2.lock().unwrap().data).create_mutex(),
        _ => { return Err(LeBlancObject::unsafe_error()); }
    };

    //println!("result {}", result.lock().unwrap().data);
    stack.push(result);

    Ok(())
}

fn _INSTRUCT_COMPARATOR_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let truth = if arg.instruct != Comparator_Else {
        match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) }
    } else {
        LeBlancObject::unsafe_error()
    };

    let block_start = handle.current_instruct;
    if arg.instruct == Comparator_Else || *truth.reflect().downcast_ref::<bool>().unwrap() {
        let jump_result = handle.execute_range(block_start + 1, block_start + 1 + arg.arg as u64);
        stack.push(jump_result);
        if arg.instruct == Comparator_Else {
            handle.current_instruct -= 1;
        } else {
            let mut jump = 0;
            while handle.current_instruct + jump < handle.instructions.len() as u64 {
                let instruct: Instruction = handle.instructions[(handle.current_instruct + jump) as usize];
                match instruct.instruct {
                    Comparator_If => jump += instruct.arg as u64,
                    Comparator_ElseIf => jump += instruct.arg as u64,
                    Comparator_Else => {
                        jump += instruct.arg as u64;
                        break;
                    }
                    _ => { jump += 1; }
                }
            }
            handle.current_instruct += jump;
        };

    } else {
        handle.current_instruct += arg.arg as u64;
    }

    Ok(())

}