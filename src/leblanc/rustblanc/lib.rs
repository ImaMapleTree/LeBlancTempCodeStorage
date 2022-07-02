




use crate::leblanc::core::module::CoreModule;

use crate::leblanc::rustblanc::lib::datelib::datelib_core_module;
use crate::leblanc::rustblanc::lib::random::random_core_module;

pub mod leblanc_colored;
pub mod datetime;
pub mod random;
pub mod datelib;


pub fn get_core_modules() -> Vec<CoreModule> {
    vec![
        random_core_module(),
        datelib_core_module()
    ]
}