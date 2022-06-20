use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::leblanc_handle::LeblancHandle;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::attributes::can_add_self;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::rustblanc::utils::Timing;

static mut TIMINGS: Option<HashMap<String, Timing>> = None;

pub unsafe fn setup_timings() {
    TIMINGS = Some(HashMap::new());
}

pub unsafe fn add_timing(name: String, duration: f64) {
    let mut timing = *TIMINGS.as_ref().unwrap().get(&name).unwrap_or(&Timing{count: 0, time: 0.0});
    timing.count += 1; timing.time += duration;
    let thash = TIMINGS.as_mut().unwrap();
    thash.insert(name.to_string(), timing);
}

pub unsafe fn print_timings() {
    println!("{:?}", TIMINGS);
}

pub fn execute_instruction(instruct: InstructionBase) -> fn(&mut LeblancHandle, &Instruction, &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    return match instruct {
        InstructionBase::BinaryAdd => _INSTRUCT_BINARY_ADD_,
        InstructionBase::InPlaceAdd => _INSTRUCT_INPLACE_ADD_,
        InstructionBase::LoadLocal => _INSTRUCT_LOAD_LOCAL_,
        InstructionBase::LoadConstant => _INSTRUCT_LOAD_CONSTANT_,
        InstructionBase::LoadFunction => _INSTRUCT_LOAD_FUNCTION_,
        InstructionBase::StoreLocal => _INSTRUCT_STORE_LOCAL_,
        InstructionBase::CallFunction => _CALL_FUNCTION_,
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
    let target = safe_stack_pop(stack, error);
    if error { return Err(target); }

    let mut targeter_unwrap = targeter.lock().unwrap();
    let mut target_unwarp = target.lock().unwrap();
    if can_add_self(&targeter_unwrap.typing) && can_add_self(&target_unwarp.typing) {
        stack.push((target_unwarp.data.as_i128() + targeter_unwrap.data.as_i128()).create_mutex());
        return Ok(());
    }

    // Things wrong with the current approach:
    // 1) Cloning the iterator is very costly
    // 2) 2 separate method calls for method tag

    //let mut matched_method: Option<Method> = target.lock().unwrap().methods.iter().cloned().filter(|m| m.has_tag(MethodTag::Addition) && m.matches("_".to_string(), &vec![targeter.lock().unwrap().to_leblanc_arg(0)])).next();
    let now = Instant::now();
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

    /*if matched_method.is_none() {
        // matched_method = targeter.lock().unwrap().methods.iter().cloned().filter(|m| m.has_tag(MethodTag::Addition) && m.matches("_".to_string(), &vec![target.lock().unwrap().to_leblanc_arg(0)])).next();

    } else {

    }*/

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
    let result= handle.variables.get(arg.arg as usize);
    if result.is_none() { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    stack.push(result.unwrap().clone());
    Ok(())
}

fn _INSTRUCT_STORE_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut Vec<Arc<Mutex<LeBlancObject>>>) -> Result<(), Arc<Mutex<LeBlancObject>>> {
    let mut error = false;
    let result = safe_stack_pop(stack, error);
    if error { return Err(Arc::new(Mutex::new(LeBlancObject::error()))); }
    if arg.arg as usize >= handle.variables.len() {
        handle.variables.push(result);
    } else {
        handle.variables[arg.arg as usize] = result;
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


    //unsafe { add_timing("CF Setup".to_string(),now.elapsed().as_secs_f64()) }
    //println!("Func: {:?}", func);

    let now = Instant::now();
    stack.push(func.clone().lock().as_mut().unwrap().data.retrieve_self_as_function().as_mut().unwrap().run_with_vec(func, &mut arguments));
    //unsafe { add_timing("CF Call".to_string(),now.elapsed().as_secs_f64()) }
    Ok(())
}