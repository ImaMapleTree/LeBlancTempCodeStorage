use alloc::rc::Rc;

use core::fmt::{Debug, Formatter};
use std::cell::RefCell;
use std::mem::swap;
use crate::leblanc::rustblanc::strawberry::Strawberry;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::internal::internal_range_generator::RangeGeneratorStepType::{ConditionalStep, FunctionStep, NegativeStep, PositiveStep};
use crate::leblanc::core::internal::transformed_iterator::TransformedIterator;
use crate::leblanc::core::leblanc_object::{LBODOperation, LeBlancObject, Reflect, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator, LeblancIterable};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::LeBlancType;

#[derive(Clone, PartialEq)]
pub struct LeblancInternalRangeGenerator {
    value: LeBlancObject,
    next_value: LeBlancObject,
    boundary: LeBlancObject,
    step: LeBlancObject,
    step_type: RangeGeneratorStepType,
    boundary_type: LeBlancType,
    reverse: bool
}

impl LeblancIterable for LeblancInternalRangeGenerator {
    fn lb_next(&mut self) -> Arc<Strawberry<LeBlancObject>> {
        match &mut self.step_type {
            PositiveStep | NegativeStep => {
                swap(&mut self.value, &mut self.next_value);
                self.next_value.data = self.value.data.simple_operation(&self.step.data, LBODOperation::BinaryAddition);
            }
            FunctionStep(method) => {
                swap(&mut self.value, &mut self.next_value);
                self.next_value.data = method.run(self.step.clone().to_mutex(), &mut [self.value.clone().to_mutex()]).lock().cast(self.boundary_type).data}
            ConditionalStep => { }
        }
        //println!("{}", self.value.to_string());
        self.value.clone().to_mutex()
    }

    fn has_next(&self) -> bool {
        match self.step_type {
            PositiveStep => self.next_value.data < self.boundary.data,
            NegativeStep => self.boundary.data < self.next_value.data,
            ConditionalStep => *self.step.reflect().downcast_ref::<bool>().unwrap(),
            _ => self.next_value.data < self.boundary.data,
        }
        //return self.value.lock().data < self.boundary.lock().data || (self.step_type == ConditionalStep && *self.step.reflect().downcast_ref::<bool>().unwrap());
    }



    fn reverse(&mut self) {
        swap(&mut self.value, &mut self.boundary);
        self.reverse = !self.reverse;
    }

    fn to_list(&mut self) -> LeblancList {
        println!("Range generator to list");
        LeblancList::new(self.collect() )
    }

    fn transformed(&mut self) -> Option<&mut TransformedIterator> { None }

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=Arc<Strawberry<LeBlancObject>>>> { Box::new(self.clone()) }
}

impl LeblancInternalRangeGenerator {
    pub fn new(value: Arc<Strawberry<LeBlancObject>>, boundary: Arc<Strawberry<LeBlancObject>>, step: Arc<Strawberry<LeBlancObject>>) -> Result<Arc<Strawberry<LeBlancObject>>, Arc<Strawberry<LeBlancObject>>> {
        let value = value.lock().clone();
        let boundary = boundary.lock().clone();
        let mut step = step.lock().clone();
        let step_type = match step.typing {
            LeBlancType::Int | LeBlancType::Int64 | LeBlancType::Int128 | LeBlancType::Short | LeBlancType::Float |
            LeBlancType::Double | LeBlancType::Arch => if step.data.as_i128() >= 0 { PositiveStep } else { NegativeStep },
            LeBlancType::String => PositiveStep,
            LeBlancType::Boolean => ConditionalStep,
            LeBlancType::Function => FunctionStep(step.data.get_mut_inner_method().unwrap().clone()),
            LeBlancType::Class(_) => { let matched_method = step.methods.iter().filter(|m| {
                m.matches("_".to_string(), &vec![value.to_leblanc_arg(0)])
                }).next().cloned();
                match matched_method {
                    None => { return Err(step.to_mutex()) }
                    Some(method) => FunctionStep(method)
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
            boundary_type,
            reverse: false
        })).to_mutex())
    }

    fn conditional_step_fn(&self) -> bool {
        *self.step.reflect().downcast_ref::<bool>().unwrap()
    }

    fn positive_step_fn(&self) -> bool {
        self.next_value.data < self.boundary.data
    }

    fn negative_step_fn(&self) -> bool {
        self.boundary.data < self.next_value.data
    }

}

#[derive(PartialEq, Clone, Debug)]
enum RangeGeneratorStepType {
    PositiveStep,
    NegativeStep,
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

impl Iterator for LeblancInternalRangeGenerator {
    type Item = Arc<Strawberry<LeBlancObject>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.has_next() {
            true => Some(self.lb_next()),
            false => None
        }

    }
}