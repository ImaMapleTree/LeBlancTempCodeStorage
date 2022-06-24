use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::internal::internal_range_generator::RangeGeneratorStepType::{ConditionalStep, FunctionStep, NormalStep};
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Reflect};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator, LeblancIterable};
use crate::LeBlancType;

pub struct LeblancInternalRangeGenerator {
    value: Rc<RefCell<LeBlancObject>>,
    boundary: Rc<RefCell<LeBlancObject>>,
    step: Rc<RefCell<LeBlancObject>>,
    step_type: RangeGeneratorStepType,
    boundary_type: LeBlancType
}

impl LeblancIterable for LeblancInternalRangeGenerator {
    fn next(&mut self) -> LeBlancObject {
        match &mut self.step_type {
            NormalStep => {self.value = self.value.call("_ADD_", &mut [self.step.clone()]).borrow().cast(self.boundary_type).to_mutex()}
            FunctionStep(method) => {self.value = method.run(self.step.clone(), &mut [self.value.clone()]).borrow().cast(self.boundary_type).to_mutex()}
            ConditionalStep => { }
        }
        return self.value.borrow()._clone()
    }

    fn has_next(&self) -> bool {
        return self.value.borrow().data < self.boundary.borrow().data || (self.step_type == ConditionalStep && *self.step.reflect().downcast_ref::<bool>().unwrap());
    }
}

impl LeblancInternalRangeGenerator {
    pub fn new(value: Rc<RefCell<LeBlancObject>>, boundary: Rc<RefCell<LeBlancObject>>, step: Rc<RefCell<LeBlancObject>>) -> Result<Rc<RefCell<LeBlancObject>>, Rc<RefCell<LeBlancObject>>> {
        let step_type = match step.borrow().typing.clone() {
            LeBlancType::Int => NormalStep,
            LeBlancType::Int64 => NormalStep,
            LeBlancType::Int128 => NormalStep,
            LeBlancType::Short => NormalStep,
            LeBlancType::Float => NormalStep,
            LeBlancType::Double => NormalStep,
            LeBlancType::Arch => NormalStep,
            LeBlancType::String => NormalStep,
            LeBlancType::Boolean => ConditionalStep,
            LeBlancType::Function => FunctionStep(step.borrow().data.get_mut_inner_method().unwrap().clone()),
            LeBlancType::Class(_) => { let matched_method = step.borrow().methods.iter().filter(|m| {
                m.matches("_".to_string(), &vec![value.borrow().to_leblanc_arg(0)])
                }).next().cloned();
                match matched_method {
                    None => { return Err(step) }
                    Some(mut method) => FunctionStep(method)
                 }
            },
            _ => return Err(step.clone())
        };
        let boundary_type = boundary.borrow().typing.clone();
        Ok(leblanc_object_iterator(Box::new(LeblancInternalRangeGenerator {
            value,
            boundary,
            step,
            step_type,
            boundary_type
        })).to_mutex())

    }
}

#[derive(PartialEq, Clone)]
enum RangeGeneratorStepType {
    NormalStep,
    FunctionStep(Method),
    ConditionalStep,
}