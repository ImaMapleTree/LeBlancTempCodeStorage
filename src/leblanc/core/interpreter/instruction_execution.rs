




use alloc::rc::Rc;
use std::cell::{BorrowError, Ref, RefCell};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use arrayvec::ArrayVec;
use crate::leblanc::core::internal::internal_range_generator::LeblancInternalRangeGenerator;

use crate::leblanc::core::interpreter::instructions::{Instruction, InstructionBase};
use crate::leblanc::core::interpreter::instructions::InstructionBase::{Comparator_Else, Comparator_ElseIf, Comparator_If};
use crate::leblanc::core::interpreter::leblanc_runner::get_globals;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, QuickUnwrap, RustDataCast};
use crate::leblanc::core::leblanc_handle::LeblancHandle;

use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::attributes::can_add_self;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::core::native_types::derived::iterator_type::LeblancIterator;
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::error_type::LeblancError;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;


use crate::{LeBlancType};
use crate::leblanc::core::native_types::double_type::leblanc_object_double;
use crate::leblanc::core::native_types::float_type::leblanc_object_float;
use crate::leblanc::core::native_types::group_type::{leblanc_object_group, LeblancGroup};


pub fn execute_instruction(instruct: InstructionBase) -> fn(&mut LeblancHandle, &Instruction, &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    match instruct {
        InstructionBase::InstructionMarker => _INSTRUCT_MARKER_,
        InstructionBase::BinaryAdd => _INSTRUCT_BINARY_ADD_,
        InstructionBase::BinarySubtract => _INSTRUCT_BINARY_SUBTRACT_,
        InstructionBase::BinaryModulo => _INSTRUCT_BINARY_MODULO_,
        InstructionBase::BinaryAnd => _INSTRUCT_BINARY_AND_,
        InstructionBase::BinaryOr => _INSTRUCT_BINARY_OR_,
        InstructionBase::InPlaceAdd => _INSTRUCT_INPLACE_ADD_,
        InstructionBase::LoadLocal => _INSTRUCT_LOAD_LOCAL_,
        InstructionBase::LoadConstant => _INSTRUCT_LOAD_CONSTANT_,
        InstructionBase::LoadFunction => _INSTRUCT_LOAD_FUNCTION_,
        InstructionBase::StoreLocal => _INSTRUCT_STORE_LOCAL_,
        InstructionBase::CallFunction => _CALL_FUNCTION_,
        InstructionBase::CallClassMethod => _INSTRUCT_CALL_CLASS_METHOD_,
        InstructionBase::IteratorSetup(_) => _INSTRUCT_CREATE_RANGE_,
        InstructionBase::ForLoop => _INSTRUCT_FOR_LOOP_,
        InstructionBase::WhileLoop => _INSTRUCT_WHILE_LOOP,
        InstructionBase::Equality(_) => _INSTRUCT_EQUALITY_,
        Comparator_If => _INSTRUCT_COMPARATOR_,
        Comparator_ElseIf => _INSTRUCT_COMPARATOR_,
        Comparator_Else => _INSTRUCT_COMPARATOR_,
        InstructionBase::ListSetup => _INSTRUCT_LIST_SETUP_,
        InstructionBase::ElementAccess => _INSTRUCT_ELEMENT_ACCESS_,
        InstructionBase::ElementStore => _INSTRUCT_ELEMENT_STORE_,
        InstructionBase::Group => _INSTRUCT_GROUP_,
        _ => _INSTRUCT_BASE_
    }
}


fn deprecated_safe_stack_pop(stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>, mut error: bool) -> Arc<Strawberry<LeBlancObject>> {
    match stack.pop() {
        None => {
            error = true;
            LeBlancObject::unsafe_error()
        }
        Some(result) => result
    }
}

fn safe_stack_pop(stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>> {
    match stack.pop() {
        None => {
            println!("Hit stack error");
            Err(LeblancError::new("UnknownStackException".to_string(), "Internal stack pop returned a none value".to_string(), vec![]).create_mutex())
        }
        Some(result) => Ok(result)
    }
}

fn _INSTRUCT_BASE_(_handle: &mut LeblancHandle, _arg: &Instruction, _stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    println!("I don't exist :)");
    Err(LeblancError::new("Instruction Doesn't Exist".to_string(), "".to_string(), vec![]).create_mutex())
}

fn _INSTRUCT_MARKER_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    stack.push(LeBlancObject::unsafe_marker());
    Ok(())
}

fn _INSTRUCT_INPLACE_ADD_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let error = false;
    let target: Arc<Strawberry<LeBlancObject>> = deprecated_safe_stack_pop(stack, error);
    if error { return Err(target); }
    let arg1 =  deprecated_safe_stack_pop(stack, error);
    if error { return Err(arg1); }


    let result = target.lock().methods.iter().cloned().find(|m| m.has_tag(MethodTag::InPlaceAddition));
        //.filter(|m| m.matches("_".to_string(), vec![tos2.lock().to_leblanc_arg(0)]))
        //.next().unwrap_or(Method::error()).run(tos1.clone(), &mut [tos2.clone()]);

    result.unwrap().run(target, &mut [arg1]);
    let result = LeBlancObject::unsafe_null();
    stack.push(result);
    Ok(())
}

fn _INSTRUCT_BINARY_ADD_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let ntargeter = targeter.lock();
    let ntarget = target.lock();


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

fn _INSTRUCT_BINARY_SUBTRACT_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let ntargeter = targeter.lock();
    let ntarget = target.lock();

    if can_add_self(&ntargeter.typing) && can_add_self(&ntarget.typing) {
        stack.push(leblanc_object_int((ntarget.data.as_i128() - ntargeter.data.as_i128()) as i32).to_mutex());
        return Ok(());
    } else {
        match ntarget.typing {
            LeBlancType::Float | LeBlancType::Double => {
                let double1: &f64 = ntarget.data.ref_data().unwrap();
                let double2: &f64 = ntargeter.data.ref_data().unwrap();
                if ntarget.typing == LeBlancType::Float {
                    stack.push(leblanc_object_float((double1 - double2) as f32).to_mutex());
                } else {
                    stack.push(leblanc_object_double(double1 - double2).to_mutex());
                }
                return Ok(());
            }
            _ => {}
        }
    }
    Ok(())
}

fn _INSTRUCT_BINARY_MODULO_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let ntargeter = targeter.lock();
    let ntarget = target.lock();

    if can_add_self(&ntargeter.typing) && can_add_self(&ntarget.typing) {
        stack.push(leblanc_object_int((ntarget.data.as_i128() % ntargeter.data.as_i128()) as i32).to_mutex());
        return Ok(());
    } else {
        match ntarget.typing {
            LeBlancType::Float | LeBlancType::Double => {
                let double1: &f64 = ntarget.data.ref_data().unwrap();
                let double2: &f64 = ntargeter.data.ref_data().unwrap();
                if ntarget.typing == LeBlancType::Float {
                    stack.push(leblanc_object_float((double1 % double2) as f32).to_mutex());
                } else {
                    stack.push(leblanc_object_double(double1 % double2).to_mutex());
                }
                return Ok(());
            }
            _ => {}
        }
    }
    Ok(())
}

fn _INSTRUCT_BINARY_AND_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    stack.push((*target.lock().data.ref_data().unwrap() && *targeter.lock().data.ref_data().unwrap()).create_mutex());
    Ok(())
}

fn _INSTRUCT_BINARY_OR_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let target = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let targeter =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    stack.push((*target.lock().data.ref_data().unwrap() || *targeter.lock().data.ref_data().unwrap()).create_mutex());
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_LOAD_FUNCTION_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let result= unsafe { get_globals() }[arg.arg as usize].clone();
    /*if result.is_none() { LeBlancObject::error().to_mutex(); }
    let result = result.unwrap().force_unwrap().to_mutex();*/
    // TODO Make method chance if we're currently running async (through a handle boolean, if so we force clone otherwise we can just check if it's locked
    stack.push(result.clone_if_locked());
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_LOAD_CONSTANT_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
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
fn _INSTRUCT_LOAD_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let result= handle.variables.get(arg.arg as usize);
    //println!("{:#?}", result);
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
            match res.lock().typing.is_numeric() {
                true => stack.push(res.clone()),
                false => stack.push(res.clone())
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn _INSTRUCT_STORE_LOCAL_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let result = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    if arg.arg as usize >= handle.variables.len() {
        handle.variables.push(result);
    } else {
        handle.variables[arg.arg as usize] = result;
    }
    Ok(())
}

#[inline(always)]
fn _CALL_FUNCTION_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
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

    //let func = func.clone_if_locked();
    let is_internal = func.lock().data.get_inner_method().unwrap().is_internal_method();
    let result = match is_internal {
        true => {
            let handle = func.lock().data.get_inner_method().unwrap().handle;
            let x = (handle)(func, &mut arguments); x
        },
        false => {//.execute(&mut arguments)//.execute(&mut arguments)
            let lock = func.lock();
            let x = lock.data.get_inner_method().unwrap().leblanc_handle.clone_if_locked().lock().execute(&mut arguments);
            x}
        };

    let typing = result.lock().typing;
    match typing {
        LeBlancType::Exception => return Err(result),
        _ => stack.push(result)
    }

    Ok(())
}

fn _INSTRUCT_CALL_CLASS_METHOD_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
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
    match object.call(method_name.lock().data.to_string().as_str(), &mut arguments) {
        Ok(result) => stack.push(result),
        Err(err) => return Err(err)
    }

    Ok(())
}

fn _INSTRUCT_CREATE_RANGE_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let increment = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let bound = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let operand = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    match LeblancInternalRangeGenerator::new(operand, bound, increment) {
        Ok(value) => stack.push(value),
        Err(value) => return Err(value)
    }

    Ok(())
}

fn _INSTRUCT_LIST_SETUP_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let mut item = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut typing = item.lock().typing;
    let mut item_list = vec![];
    while typing != LeBlancType::Marker {
        item_list.push(item);
        item = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
        typing = item.lock().typing;
    }
    item_list.reverse();
    stack.push(LeblancList::new(item_list, ).create_mutex());
    Ok(())
}

fn _INSTRUCT_WHILE_LOOP(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let mut instructs = vec![];
    let mut instruct = *handle.instructions.get((handle.current_instruct+1) as usize).unwrap();
    let mut i = 1;
    let loop_start = handle.current_instruct;
    while instruct.instruct != InstructionBase::InstructionMarker {
        instructs.push(instruct);
        i += 1;
        instruct = *handle.instructions.get((handle.current_instruct+i) as usize).unwrap();
    }
    let jump = arg.arg - ((instructs.len()-1) as u16);

    let truth = handle.execute_instructions(&instructs, stack);
    let mut boolean: bool = *truth.lock().data.ref_data().unwrap();
    while boolean {
        let _loop_result = handle.execute_range(loop_start+1, loop_start+1 + arg.arg as u64 );
        let truth = handle.execute_instructions(&instructs, stack);
        boolean = *truth.lock().data.ref_data().unwrap();

    }

    handle.current_instruct = loop_start;
    handle.current_instruct += jump as u64;

    Ok(())

}

fn _INSTRUCT_FOR_LOOP_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let iter_variable = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut iterable = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let loop_start = handle.current_instruct;

    if iterable.lock().typing != LeBlancType::Derived(DerivedType::Iterator) {
        iterable = match iterable.call_name("iterate") {
            Ok(iter) => iter,
            Err(err) => return Err(err)
        }
    }

    let mut borrowed_iterable = iterable.lock();
    let inner_iterator: &mut LeblancIterator = borrowed_iterable.data.mut_data().unwrap();

    while inner_iterator.has_next() {
        let variable = inner_iterator.next();
        //println!("I'm good");
        let mut lock = iter_variable.lock();
        lock.move_data(variable.arc_unwrap());
        //println!("I'm goododeer");
        let _loop_result = handle.execute_range(loop_start+1, loop_start+1 + arg.arg as u64 );
        //println!("Hahahaha");
        //variable.lock().swap_rc(&mut iter_variable.lock());
        //println!("I'm bad");
    }

    handle.current_instruct = loop_start;
    handle.current_instruct += arg.arg as u64;

    Ok(())
}

fn _INSTRUCT_EQUALITY_(_handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    //println!("Error?");
    let tos1 = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let tos2 = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    //println!("Good bye cruel world");

    let tos1 = tos1.lock();
    let tos2_borrow = tos2.lock();

    stack.push(match arg.arg {
        0 => (tos1.data == tos2_borrow.data),
        1 => (tos1.data != tos2_borrow.data),
        2 => (tos1.data > tos2_borrow.data),
        3 => (tos1.data < tos2_borrow.data),
        4 => (tos1.data >= tos2_borrow.data),
        5 => (tos1.data <= tos2_borrow.data),
        _ => { return Err(LeBlancObject::unsafe_error()); }
    }.create_mutex());

    Ok(())
}

fn _INSTRUCT_COMPARATOR_(handle: &mut LeblancHandle, arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let truth = if arg.instruct != Comparator_Else {
        match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) }
    } else {
        LeBlancObject::unsafe_error()
    };

    let block_start = handle.current_instruct;
    if arg.instruct == Comparator_Else || *truth.lock().data.ref_data().unwrap() {
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

fn _INSTRUCT_ELEMENT_ACCESS_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let list_like = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let accessor = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let mut borrowed = list_like.lock();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();

    let accessor_type = accessor.lock().typing;
    if accessor_type == LeBlancType::Derived(DerivedType::Slice) {

    } else {
        let index = accessor.lock().data.as_i128() as usize;
        stack.push(match list.internal_vec.get(index) {
            None => return Err(LeblancError::new("IndexOutOfBoundsException".to_string(), format!("Cannot access an element at index: {} when object length is: {}", index, list.internal_vec.len()), vec![]).create_mutex()),
            Some(e) => e.clone(),
        });
    }
    Ok(())

}

fn _INSTRUCT_ELEMENT_STORE_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let list_like = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let accessor = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let value = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };

    let mut borrowed = list_like.lock();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();

    let accessor_type = accessor.lock().typing;
    if accessor_type == LeBlancType::Derived(DerivedType::Slice) {

    } else {
        let index = accessor.lock().data.as_i128() as usize;
        list.internal_vec[index] = value;
    }
    Ok(())
}

fn _INSTRUCT_GROUP_(_handle: &mut LeblancHandle, _arg: &Instruction, stack: &mut ArrayVec<Arc<Strawberry<LeBlancObject>>, 80>) -> Result<(), Arc<Strawberry<LeBlancObject>>> {
    let target =  match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let group = match safe_stack_pop(stack) { Ok(res) => res, Err(err) => return Err(err) };
    let mut group_borrow = group.lock();

    if group_borrow.typing == LeBlancType::Null {
        let group = leblanc_object_group(LeblancGroup::default());
        group_borrow.move_data(group);
    }

    let leblanc_group: &mut LeblancGroup = group_borrow.data.mut_data().unwrap();

    stack.push(leblanc_group.promise(target).create_mutex());

    Ok(())
}