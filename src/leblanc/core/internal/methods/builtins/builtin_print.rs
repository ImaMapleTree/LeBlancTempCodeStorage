use std::collections::BTreeSet;
use std::io;
use std::sync::{Arc, Mutex};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::{Callable, LeBlancObject, Stringify};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::native_types::base_type::internal_method;
use crate::leblanc::rustblanc::utils::Timings;
use crate::LeBlancType;

static mut TIMING: Timings = Timings { map: None };

static mut STDOUT: Option<io::Stdout> = None;

pub unsafe fn setup_timings() {
    TIMING.setup();
}

pub unsafe fn print_timings() {
    TIMING.print_timing();
}

fn _BUILTIN_PRINT_(_self: Arc<Mutex<LeBlancObject>>, args: &mut [Arc<Mutex<LeBlancObject>>]) -> Arc<Mutex<LeBlancObject>> {
    //let now = Instant::now();

    let result = args[0].call_name("toString");
    //let result = args[0].call("toString", &mut []);
    //unsafe { TIMING.add_timing("toStringCall".to_string(), now.elapsed().as_secs_f64()); }
    //let now = Instant::now();
    let unlock = result.to_string() + "\n";
    //unsafe { TIMING.add_timing("unlock".to_string(), now.elapsed().as_secs_f64()); }
    //let now = Instant::now();
    //println!("{}", unlock);
    unsafe {
        if STDOUT.is_none() {
            STDOUT = Some(io::stdout());
        }
        io::copy(&mut unlock.as_bytes(), &mut STDOUT.as_mut().unwrap()).unwrap();
    }
    //stdout().write_all(&unlock.into_bytes()).unwrap();
    //unsafe { TIMING.add_timing("print".to_string(), now.elapsed().as_secs_f64()); }
    return LeBlancObject::unsafe_null()
}

pub fn _BUILTIN_PRINT_METHOD_() -> Method {
    Method::new(
        MethodStore::new(
            "print".to_string(),
            vec![LeBlancArgument::default(LeBlancType::Flex, 0)]
        ),
        _BUILTIN_PRINT_,
        BTreeSet::new()
    )
}

pub fn _BUILTIN_PRINT_OBJECT_() -> LeBlancObject {
    return internal_method(_BUILTIN_PRINT_METHOD_());
}