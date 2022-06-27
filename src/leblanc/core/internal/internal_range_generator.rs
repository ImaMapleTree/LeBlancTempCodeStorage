use alloc::rc::Rc;
use core::fmt::{Debug, Formatter};
use std::cell::RefCell;
use std::mem::swap;
use crate::leblanc::core::internal::internal_range_generator::RangeGeneratorStepType::{ConditionalStep, FunctionStep, NormalStep};
use crate::leblanc::core::leblanc_object::{Callable, LBODOperation, LeBlancObject, Reflect, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator, LeblancIterable};
use crate::LeBlancType;

#[derive(Clone, PartialEq)]
pub struct LeblancInternalRangeGenerator {
    value: LeBlancObject,
    next_value: LeBlancObject,
    boundary: LeBlancObject,
    step: LeBlancObject,
    step_type: RangeGeneratorStepType,
    boundary_type: LeBlancType
}

impl LeblancIterable for LeblancInternalRangeGenerator {
    fn next(&mut self) -> Rc<RefCell<LeBlancObject>> {
        match &mut self.step_type {
            NormalStep => {
                swap(&mut self.value, &mut self.next_value);
                let data = self.value.data.simple_operation(&self.step.data, LBODOperation::BinaryAddition);
                self.next_value.data = data;

            }
            FunctionStep(method) => {self.value = method.run(self.step.clone().to_mutex(), &mut [self.value.clone().to_mutex()]).borrow().cast(self.boundary_type)}
            ConditionalStep => { }
        }
        //println!("{}", self.value.to_string());
        return self.value._clone().to_mutex()
    }

    fn has_next(&self) -> bool {
        match self.step_type {
            ConditionalStep => *self.step.reflect().downcast_ref::<bool>().unwrap(),
            _ => { self.next_value.data < self.boundary.data}
        }
        //return self.value.borrow().data < self.boundary.borrow().data || (self.step_type == ConditionalStep && *self.step.reflect().downcast_ref::<bool>().unwrap());
    }
}

impl LeblancInternalRangeGenerator {
    pub fn new(value: Rc<RefCell<LeBlancObject>>, boundary: Rc<RefCell<LeBlancObject>>, step: Rc<RefCell<LeBlancObject>>) -> Result<Rc<RefCell<LeBlancObject>>, Rc<RefCell<LeBlancObject>>> {
        let value = value.borrow().clone();
        let boundary = boundary.borrow().clone();
        let mut step = step.borrow().clone();
        let step_type = match step.typing {
            LeBlancType::Int => NormalStep,
            LeBlancType::Int64 => NormalStep,
            LeBlancType::Int128 => NormalStep,
            LeBlancType::Short => NormalStep,
            LeBlancType::Float => NormalStep,
            LeBlancType::Double => NormalStep,
            LeBlancType::Arch => NormalStep,
            LeBlancType::String => NormalStep,
            LeBlancType::Boolean => ConditionalStep,
            LeBlancType::Function => FunctionStep(step.data.get_mut_inner_method().unwrap().clone()),
            LeBlancType::Class(_) => { let matched_method = step.methods.iter().filter(|m| {
                m.matches("_".to_string(), &vec![value.to_leblanc_arg(0)])
                }).next().cloned();
                match matched_method {
                    None => { return Err(step.to_mutex()) }
                    Some(mut method) => FunctionStep(method)
                 }
            },
            _ => return Err(step.to_mutex())
        };
        let boundary_type = boundary.typing;
        let value = value.cast(boundary_type);
        let next_value = value.clone();
        Ok(leblanc_object_iterator(Box::new(LeblancInternalRangeGenerator {
            value,
            next_value,
            boundary,
            step,
            step_type,
            boundary_type
        })).to_mutex())

    }
}

#[derive(PartialEq, Clone, Debug)]
enum RangeGeneratorStepType {
    NormalStep,
    FunctionStep(Method),
    ConditionalStep,
}

impl Debug for LeblancInternalRangeGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LeblancInternalRangeGenerator")
            .field("value", &self.value.data.to_string())
            .field("boundary", &self.boundary.data.to_string())
            .field("step", &self.step.data.to_string())
            .finish()
    }
}