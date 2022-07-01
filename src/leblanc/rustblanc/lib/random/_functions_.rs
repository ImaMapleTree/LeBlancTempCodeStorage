use alloc::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::time::Instant;
use chrono::Local;
use rand_chacha::{ChaCha12Rng, ChaCha8Rng};
use rand_chacha::rand_core::{RngCore, SeedableRng};
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::LeBlancType;

static mut RNG_GENERATOR: Option<ChaCha8Rng> = None;

fn initialize_generator() -> &'static mut Option<ChaCha8Rng> {
    unsafe {
        if RNG_GENERATOR.is_none() { RNG_GENERATOR = Some(ChaCha8Rng::seed_from_u64(Local::now().timestamp() as u64)) }
        &mut RNG_GENERATOR
    }
}

fn random_number() -> f64 {
    let generator = initialize_generator().as_mut().unwrap();
    (generator.next_u64() as f64).div(u64::MAX as f64)
}

pub fn _random_no_arg_(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    random_number().create_mutex()
}

pub fn _random_one_arg_(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let borrowed = args[0].borrow();
    let value = borrowed.data.as_i128();

    let random_value = random_number().mul(value as u64 as f64);
    match borrowed.typing {
        LeBlancType::Float | LeBlancType::Double => random_value.create(),
        _ => (random_value.ceil() as i128).create().cast(borrowed.typing)
    }.to_mutex()
}

pub fn _random_two_arg_(_self: Rc<RefCell<LeBlancObject>>, args: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let borrowed1 = args[0].borrow();
    let borrowed2 = args[1].borrow();
    let lower_bound = borrowed1.data.as_i128();
    let upper_bound = borrowed2.data.as_i128();

    let random_value = random_number().mul((upper_bound - lower_bound) as u64 as f64).add(lower_bound as u64 as f64);
    match borrowed1.typing {
        LeBlancType::Float | LeBlancType::Double => random_value.create(),
        _ => (random_value.ceil() as i128).create().cast(borrowed1.typing)
    }.to_mutex()
}

