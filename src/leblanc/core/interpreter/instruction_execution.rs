




use alloc::rc::Rc;
use std::cell::RefCell;
use arrayvec::ArrayVec;
use crate::leblanc::core::internal::internal_range_generator::LeblancInternalRangeGenerator;

use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::instructions::InstructionBase::{Comparator_Else, Comparator_ElseIf, Comparator_If};
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect, RustDataCast};
use crate::leblanc::core::leblanc_handle::LeblancHandle;

use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::attributes::can_add_self;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::core::native_types::derived::iterator_type::LeblancIterator;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::int128_type::leblanc_object_int128;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;


use crate::LeBlancType;



pub fn execute_instruction(instruct: InstructionBase) -> fn(&mut LeblancHandle, &Instruction, &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    match instruct {
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


fn deprecated_safe_stack_pop(stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>, mut error: bool) -> Rc<RefCell<LeBlancObject>> {
    match stack.pop() {
        None => {
            error = true;
            LeBlancObject::unsafe_error()
        }
        Some(result) => result
    }
}

fn safe_stack_pop(stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<Rc<RefCell<LeBlancObject>>, Rc<RefCell<LeBlancObject>>> {
    match stack.pop() {
        None => {
            println!("Hit stack error");
            Err(LeblancError::new("UnknownStackException".to_string(), "Internal stack pop returned a none value".to_string(), vec![]).create_mutex())
        }
        Some(result) => Ok(result)
    }
}

fn _INSTRUCT_BASE_(_handle: &mut LeblancHandle, _arg: &Instruction, _stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    Err(LeBlancObject::unsafe_error())
}

fn _INSTRUCT_INPLACE_ADD_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let error = false;
    let target: Rc<RefCell<LeBlancObject>> = deprecated_safe_stack_pop(stack, error);
    if error { return Err(target); }
    let arg1 =  deprecated_safe_stack_pop(stack, error);
    if error { return Err(arg1); }


    let result = target.borrow_mut().methods.iter().cloned().find(|m| m.has_tag(MethodTag::InPlaceAddition));
        //.filter(|m| m.matches("_".to_string(), vec![tos2.lock().unwrap().to_leblanc_arg(0)]))
        //.next().unwrap_or(Method::error()).run(tos1.clone(), &mut [tos2.clone()]);

    result.unwrap().run(target, &mut [arg1]);
    let result = LeBlancObject::unsafe_null();
    stack.push(result);
    Ok(())
}

fn _INSTRUCT_BINARY_ADD_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let ntargeter = targeter.borrow();
    let ntarget = target.borrow();


    if can_add_self(&ntargeter.typing) && can_add_self(&ntarget.typing) {
        stack.push(leblanc_object_int((ntarget.data.as_i128() + ntargeter.data.as_i128()) as i32).to_mutex());
        return Ok(());
    }

    let arguments = vec![ntargeter.to_leblanc_arg(0)];
    let matched_method = ntarget.methods.iter().filter(|m| {
        m.matches("_".to_string(), &arguments)
    }).next().cloned();
    match matched_method {
        None => {
            let arguments = vec![ntarget.to_leblanc_arg(0)];
            let matched_method = ntargeter.methods.iter().filter(|m| {
                m.matches("_".to_string(), &arguments)
            }).next().cloned();
            if matched_method.is_none() {
                return Err(LeBlancObject::unsafe_error());
            }
            drop(ntargeter);
            drop(ntarget);
            stack.push(matched_method.unwrap().run(targeter, &mut [target]));
        }
        Some(mut method) => {
            drop(ntargeter);
            drop(ntarget);
            stack.push(method.run(target, &mut [targeter]));
        }
    }

    Ok(())
}

fn _INSTRUCT_BINARY_SUBTRACT_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let ntargeter = targeter.borrow();
    let ntarget = target.borrow();


    if can_add_self(&ntargeter.typing) && can_add_self(&ntarget.typing) {
        stack.push(leblanc_object_int((ntarget.data.as_i128() - ntargeter.data.as_i128()) as i32).to_mutex());
        return Ok(());
    }

    let arguments = vec![ntargeter.to_leblanc_arg(0)];
    let matched_method = ntarget.methods.iter().filter(|m| {
        m.matches("_".to_string(), &arguments)
    }).next().cloned();
    match matched_method {
        None => {
            let arguments = vec![ntarget.to_leblanc_arg(0)];
            let matched_method = ntargeter.methods.iter().filter(|m| {
                m.matches("_".to_string(), &arguments)
            }).next().cloned();
            if matched_method.is_none() {
                return Err(LeBlancObject::unsafe_error());
            }
            drop(ntargeter);
            drop(ntarget);
            stack.push(matched_method.unwrap().run(targeter, &mut [target]));
        }
        Some(mut method) => {
            drop(ntargeter);
            drop(ntarget);
            stack.push(method.run(target, &mut [targeter]));
        }
    }

    Ok(())
}

#[inline(always)]
fn _INSTRUCT_LOAD_FUNCTION_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let result= unsafe { get_globals() }.get(arg.arg as usize);
    if result.is_none() { LeBlancObject::error().to_mutex(); }
    stack.push(result.unwrap().clone());
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_LOAD_CONSTANT_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let result= handle.constants.get(arg.arg as usize);
    match result {
        None => Err(LeBlancObject::error().to_mutex()),
        Some(constant) => {
            stack.push(constant.clone());
            Ok(())
        }
    }

}

#[inline(always)]
fn _INSTRUCT_LOAD_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let result= handle.variables.get(arg.arg as usize);
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
            match res.borrow().typing.is_numeric() {
                true => stack.push(res.clone()),
                false => stack.push(res.clone())
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_STORE_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let result = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    if arg.arg as usize >= handle.variables.len() {
        handle.variables.push(result);
    } else {
        handle.variables[arg.arg as usize] = result;
    }
    Ok(())
}

#[inline(always)]
fn _CALL_FUNCTION_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
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


    let is_internal = func.borrow().data.get_inner_method().unwrap().is_internal_method();
    let result = ( match is_internal {
        true => {
            let x = (func.clone().borrow().data.get_inner_method().unwrap().handle)(func, &mut arguments); x
        },
        false => {
            match func.borrow().data.get_inner_method().unwrap().leblanc_handle.try_borrow_mut() {
                Ok(mut refer) => refer.execute(&mut arguments),
                Err(_err) => unsafe {&mut (*func.borrow().data.get_inner_method().unwrap().leblanc_handle.as_ptr())}.clone().execute(&mut arguments)
            }}
        });

    let typing = result.borrow().typing;
    match typing {
        LeBlancType::Exception => return Err(result),
        _ => stack.push(result)
    }

    Ok(())
}

fn _INSTRUCT_CALL_CLASS_METHOD_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
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
    //println!("object: {:#?}", object);
    if error { return Err(object); }
    match object.call(method_name.borrow().data.to_string().as_str(), &mut arguments) {
        Ok(result) => stack.push(result),
        Err(err) => return Err(err)
    }

    Ok(())
}

fn _INSTRUCT_CREATE_RANGE_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let increment = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let operand = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let bound = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };


    match LeblancInternalRangeGenerator::new(operand, bound, increment) {
        Ok(value) => stack.push(value),
        Err(value) => return Err(value)
    }

    Ok(())
}

fn _INSTRUCT_FOR_LOOP_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let iter_variable = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut iterable = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let loop_start = handle.current_instruct;

    // make into an iterator eventually, right now we'll just grab the internal list
    if iterable.borrow().typing != LeBlancType::Derived(DerivedType::Iterator) {
        iterable = match iterable.call_name("iterator") {
            Ok(iter) => iter,
            Err(err) => return Err(err)
        }
    }

    /*let mut reflection = iterable.reflect();
    let inner_iterator = reflection.downcast_mut::<LeblancIterator>().unwrap();*/
    let mut borrowed_iterable = iterable.borrow_mut();
    let inner_iterator: &mut LeblancIterator = borrowed_iterable.data.mut_data().unwrap();


    while inner_iterator.has_next() {
        iter_variable.borrow_mut().move_data(inner_iterator.next());
        let _loop_result = handle.execute_range(loop_start+1, loop_start+1 + arg.arg as u64 );
    }

    //TODO: Create iterator instead of an array, list comprehension

    //println!("Done with loop");
    handle.current_instruct = loop_start;
    handle.current_instruct += arg.arg as u64;

    Ok(())
}

fn _INSTRUCT_EQUALITY_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    //println!("Error?");
    let tos1 = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let tos2 = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    //println!("Good bye cruel world");

    let tos1 = tos1.borrow();
    let tos2_borrow = tos2.borrow();

    let result = match arg.arg {
        0 => (tos1.data == tos2_borrow.data),
        1 => (tos1.data != tos2_borrow.data),
        2 => (tos1.data > tos2_borrow.data),
        3 => (tos1.data < tos2_borrow.data),
        4 => (tos1.data >= tos2_borrow.data),
        5 => (tos1.data <= tos2_borrow.data),
        _ => { return Err(LeBlancObject::unsafe_error()); }
    };

    //println!("result {}", result.loan().inquire().either().data);
    stack.push(result.create_mutex());

    Ok(())
}

fn _INSTRUCT_COMPARATOR_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Rc<RefCell<LeBlancObject>>, 80>) -> Result<(), Rc<RefCell<LeBlancObject>>> {
    let truth = if arg.instruct != Comparator_Else {
        match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) }
    } else {
        LeBlancObject::unsafe_error()
    };

    let block_start = handle.current_instruct;
    if arg.instruct == Comparator_Else || *truth.borrow().data.ref_data().unwrap() {
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