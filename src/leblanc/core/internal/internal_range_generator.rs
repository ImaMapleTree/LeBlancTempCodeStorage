

use core::fmt::{Debug, Formatter};

use std::mem::swap;


use crate::leblanc::core::internal::internal_range_generator::RangeGeneratorStepType::{ConditionalStep, FunctionStep, NegativeStep, PositiveStep};
use crate::leblanc::core::internal::transformed_iterator::TransformedIterator;
use crate::leblanc::core::leblanc_object::{Reflect, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator, LeblancIterable};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};
use crate::unsafe_vec;

#[derive(Clone, PartialEq)]
pub struct LeblancInternalRangeGenerator {
    value: LBObject,
    next_value: LBObject,
    boundary: LBObject,
    step: LBObject,
    step_type: RangeGeneratorStepType,
    boundary_type: LeBlancType,
    reverse: bool
}

impl LeblancIterable for LeblancInternalRangeGenerator {
    fn lb_next(&mut self) -> LBObject {
        match &mut self.step_type {
            PositiveStep | NegativeStep => {
                swap(&mut self.value, &mut self.next_value);
                //self.next_value.data = self.value.data.simple_operation(&self.step.data, LBODOperation::BinaryAddition);
                // TODO
            }
            FunctionStep(method) => {
                swap(&mut self.value, &mut self.next_value);
                self.next_value.data = method.run(self.step.clone(), unsafe_vec![self.value.clone()]).cast(self.boundary_type).data.clone()}
            ConditionalStep => { }
        }
        //println!("{}", self.value.to_string());
        self.value.clone()
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

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=LBObject>> { Box::new(self.clone()) }
}

impl LeblancInternalRangeGenerator {
    pub fn new(value: LBObject, boundary: LBObject, step: LBObject) -> Result<LBObject, LBObject> {
        let value = value;
        let boundary = boundary.to_owned();
        let mut step = step.to_owned();
        let step_type = match LeBlancType::from_enum_id(step.typing as u16) {
            LeBlancType::Int | LeBlancType::Int64 | LeBlancType::Int128 | LeBlancType::Short | LeBlancType::Float |
            LeBlancType::Double | LeBlancType::Arch => if step.data.as_i128() >= 0 { PositiveStep } else { NegativeStep },
            LeBlancType::String => PositiveStep,
            LeBlancType::Boolean => ConditionalStep,
            LeBlancType::Function => FunctionStep(step.data.get_mut_inner_method().unwrap().clone()),
            _ => return Err(step)
        };
        let boundary_type = LeBlancType::from_enum_id(boundary.typing as u16);
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
        })))
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
    type Item = LBObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self.has_next() {
            true => Some(self.lb_next()),
            false => None
        }

    }
}