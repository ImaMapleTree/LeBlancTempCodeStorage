use core::borrow::BorrowMut;
use std::collections::HashMap;
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


fn safe_stack_pop(stack: &mut Vec<Arc<Mutex<LeBlancObject>>>, mut error: bool) -> Arc<Mutex<LeBlancObject>> {
    let stack_result = stack.pop();
    return match stack_result {
        None => {
            error = true;
            LeBlancObject::unsafe_error()
        }
        Some(result) => result
    };
}

fn _INSTRUCT_BASE_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    Err(Arc::new(Mutex::new(LeBlancObject::error())))
}

fn _INSTRUCT_INPLACE_ADD_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let target = safe_stack_pop(stack, error);
    if error { return Err(target); }
    let arg1 =  safe_stack_pop(stack, error);
    if error { return Err(arg1); }


    let result = target.lock().unwrap().methods.iter().cloned().filter(|m| m.has_tag(MethodTag::InPlaceAddition)).next();
        //.filter(|m| m.matches("_".to_string(), vec![tos2.lock().unwrap().to_leblanc_arg(0)]))
        //.next().unwrap_or(Method::error()).run(tos1.clone(), &mut [tos2.clone()]);

    result.unwrap().run(target.clone(), &mut [arg1.clone()]);
    let result = Arc::new(Mutex::new(LeBlancObject::null()));
    stack.push(result);
    Ok(())
}

#[inline]
fn _INSTRUCT_BINARY_ADD_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let targeter =  safe_stack_pop(stack, error);
    if error { return Err(targeter); }
    let mut target = safe_stack_pop(stack, error);
    if error { return Err(target); }

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
            let now = Instant::now();
            stack.push(matched_method.unwrap().run(targeter_clone, &mut [target_clone]));
        }
        Some(mut method) => {
            let now = Instant::now();
            stack.push(method.run(target_clone, &mut [targeter.clone()]));
        }
    }

    Ok(())
}

fn _INSTRUCT_BINARY_SUBTRACT_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let mut target =  safe_stack_pop(stack, error);
    if error { return Err(target); }
    let targeter = safe_stack_pop(stack, error);
    if error { return Err(targeter); }

    let mut targeter_unwrap = targeter.lock().unwrap();
    if Mutex::try_lock(&target).is_err() {
        target = Arc::new(Mutex::new(targeter_unwrap._clone()));
    }
    let mut target_unwarp = target.lock().unwrap();

    //println!("Target: {:?}", target_unwarp);
    //println!("Targeter: {:?}", targeter_unwrap);

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
            ////unsafe { add_timing("BA - Matching".to_string(),now.elapsed().as_secs_f64()) }
            if matched_method.is_none() {
                return Err(Arc::new(Mutex::new(LeBlancObject::error())));
            }
            let now = Instant::now();
            stack.push(matched_method.unwrap().run(targeter_clone, &mut [target_clone]));
            ////unsafe { add_timing("BA - Executing".to_string(),now.elapsed().as_secs_f64()) }
        }
        Some(mut method) => {
            ////unsafe { add_timing("BA - Matching".to_string(),now.elapsed().as_secs_f64()) }
            let now = Instant::now();
            stack.push(method.run(target_clone, &mut [targeter.clone()]));
            ////unsafe { add_timing("BA - Executing".to_string(),now.elapsed().as_secs_f64()) }
        }
    }

    Ok(())
}


fn _INSTRUCT_LOAD_FUNCTION_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= unsafe { get_globals() }.get(arg.arg as usize);
    if result.is_none() { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    stack.push(result.unwrap().clone());
    Ok(())
}

#[inline]
fn _INSTRUCT_LOAD_CONSTANT_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= handle.constants.get(arg.arg as usize);
    match result {
        None => return Err(Arc::new(Mutex::new(LeBlancObject::error()))),
        Some(constant) => {
            let constant_clone = Arc::new(Mutex::new(constant._clone()));
            stack.push(constant_clone);
            Ok(())
        }
    }

}

fn _INSTRUCT_LOAD_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let result= handle.variables.lock().unwrap().get(arg.arg as usize).cloned();
    //if result.is_none() { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    match result {
        None => {
            let null = Arc::new(Mutex::new(LeBlancObject::null()));
            if arg.arg as usize >= handle.variables.lock().unwrap().len() {
                handle.variables.lock().unwrap().push(null.clone());
            } else {
                handle.variables.lock().unwrap()[arg.arg as usize] = null.clone();
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

fn _INSTRUCT_STORE_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut error = false;
    let result = safe_stack_pop(stack, error);
    if error { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    if arg.arg as usize >= handle.variables.lock().unwrap().len() {
        handle.variables.lock().unwrap().push(result);
    } else {
        handle.variables.lock().unwrap()[arg.arg as usize] = result;
    }
    Ok(())
}

fn _CALL_FUNCTION_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let now = Instant::now();
    let error = false;
    let func = safe_stack_pop(stack, error);
    if error { return Err(func); }
    let mut arguments = match arg.arg {
        0 => { Vec::new() }
        1 => { vec![safe_stack_pop(stack, error)] }
        _ => {
            let mut vector = Vec::new();
            for _ in 0..arg.arg as usize {
                let tos = safe_stack_pop(stack, error);
                if error { return Err(tos); }
                vector.push(tos);
            }
            vector.reverse();
            vector
        }
    };

    let mut is_leblanc_handle = true;
    let mut mutex = func.lock().unwrap();
    let mutex_method = mutex.data.retrieve_self_as_function().unwrap();
    if mutex_method.leblanc_handle.null {
        is_leblanc_handle = false;
    }

    let result = match is_leblanc_handle {
        true => {
            let mut leblanc_handle = mutex_method.leblanc_handle.clone();
            Mutex::unlock(mutex);
            leblanc_handle.execute(Arc::new(Mutex::new(arguments.to_vec())))
        }
        false => {
            let method = mutex_method.handle;
            Mutex::unlock(mutex);
            (method)(func.clone(), &mut arguments)
        }
    };

    //let mut method = func.lock().unwrap().data.retrieve_self_as_function().unwrap().clone();

    //stack.push( method.run_with_vec(func, &mut arguments));

    stack.push(result);

    Ok(())
}

fn _INSTRUCT_CALL_CLASS_METHOD_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let method_name = safe_stack_pop(stack, error);
    if error { return Err(method_name); }
    let mut arguments = match arg.arg {
        0 => { Vec::new() }
        1 => { vec![safe_stack_pop(stack, error)] }
        _ => {
            let mut vector = Vec::new();
            for _ in 0..arg.arg as usize {
                let tos = safe_stack_pop(stack, error);
                if error { return Err(tos); }
                vector.push(tos);
            }
            vector.reverse();
            vector
        }
    };
    let mut object = safe_stack_pop(stack, error);
    if error { return Err(object); }
    stack.push(object.call(method_name.lock().unwrap().data.to_string().as_str(), &mut arguments));
    Ok(())
}

fn _INSTRUCT_CREATE_RANGE_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let mut increment = safe_stack_pop(stack, error);
    if error { return Err(increment) }
    let mut operand = safe_stack_pop(stack, error);
    if error { return Err(operand) }
    let bound = safe_stack_pop(stack, error);
    if error { return Err(bound) }


    let mut internal_list = LeblancList::empty();

    /*println!("Operand: {:?}", operand);
    println!("Bound: {:?}", bound);
    println!("Increment: {:?}", increment);*/
    while operand.lock().unwrap().data < bound.lock().unwrap().data || (*&increment.leblanc_type() == LeBlancType::Boolean && *increment.reflect().downcast_ref::<bool>().unwrap()){
        //println!("{}", internal_list);
        internal_list.internal_vec.push(Arc::new(Mutex::new(operand.lock().unwrap()._clone())));
        if increment.leblanc_type() == LeBlancType::Function {
            operand = Arc::new(Mutex::new(increment.lock().unwrap().data.retrieve_self_as_function().unwrap().run(increment.clone(), &mut [operand]).lock().unwrap()._clone().cast(bound.leblanc_type())));
        } else if operand.leblanc_type().is_native() {
            operand = Arc::new(Mutex::new(operand.call("_ADD_", &mut [increment.clone()]).lock().unwrap()._clone().cast(bound.leblanc_type())));
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
    let error = false;
    let mut iter_variable = safe_stack_pop(stack, error);
    if error { return Err(iter_variable) }
    let iterable = safe_stack_pop(stack, error);
    if error { return Err(iterable) }

    let loop_start = handle.current_instruct;

    // make into an iterator eventually, right now we'll just grab the internal list
    for item in &iterable.reflect().downcast_ref::<LeblancList>().unwrap().internal_vec {
        iter_variable.lock().unwrap().copy_data(&item.lock().unwrap());
        let loop_result = handle.execute_range(loop_start+1, loop_start+1 + arg.arg as u64 );
        //println!("Loop result: {:?}", loop_result);
        //stack.push();
    }

    //println!("Done with loop");
    handle.current_instruct = loop_start;
    handle.current_instruct += arg.arg as u64;

    Ok(())

}

fn _INSTRUCT_EQUALITY_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let error = false;
    let tos1 = safe_stack_pop(stack, error);
    if error { return Err(tos1) }
    let tos2 = safe_stack_pop(stack, error);
    if error { return Err(tos2) }

    let tos1 = Arc::new(Mutex::new(tos1.lock().unwrap()._clone().cast(tos2.leblanc_type())));

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
    let error = false;
    let truth = if arg.instruct != Comparator_Else {
        safe_stack_pop(stack, error)
    } else {
        LeBlancObject::unsafe_error()
    };
    if error { return Err(truth); }

    let block_start = handle.current_instruct;
    //println!("Block start: {}", block_start);
    if arg.instruct == Comparator_Else || *truth.reflect().downcast_ref::<bool>().unwrap() {
        let jump_result =handle.execute_range(block_start + 1, block_start + 1 + arg.arg as u64);
        //println!("Jump result: {:?}", jump_result);
        stack.push(jump_result);
        if arg.instruct == Comparator_Else {
            handle.current_instruct -= 1;
        } else {
            let mut jump = 0;
            while arg.instruct != Comparator_Else && handle.current_instruct + jump < handle.instructions.len() as u64 {
                //println!("test: {} | {:?}", test, handle.instructions[(handle.current_instruct + test) as usize]);
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
    //println!("Jumping");

    Ok(())

}