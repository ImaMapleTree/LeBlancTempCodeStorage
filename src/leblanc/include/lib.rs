




use crate::leblanc::core::module::CoreModule;
use crate::leblanc::include::lib::io::io_core_module;

use crate::leblanc::include::lib::timelib::datelib_core_module;
use crate::leblanc::include::lib::random::random_core_module;

pub mod leblanc_colored;
pub mod datetime;
pub mod random;
pub mod timelib;
pub mod io;


pub fn get_core_modules() -> Vec<CoreModule> {
    vec![
        random_core_module(),
        datelib_core_module(),
        io_core_module()
    ]
}