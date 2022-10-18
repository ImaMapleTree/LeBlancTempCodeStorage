// 3 + 2





use std::mem::size_of_val;
use std::time::Instant;
use bumpalo::Bump;
use lazy_static::lazy_static;
use crate::leblanc::core::interpreter::HEAP;
use crate::leblanc::core::leblanc_object::{LeBlancObject};
use crate::leblanc::core::native_types::double_type::leblanc_object_double;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::blueberry::{Blueberry, BlueberryVec};
use crate::leblanc::rustblanc::heap::{Heap, HeapRef};
use crate::leblanc::rustblanc::strawberry::Strawberry;




fn test() -> LeBlancObject {
    let run_amount = 100;
    let mut v: Vec<HeapRef<LeBlancObject>> = vec![];
    let mut v2: Vec<&mut LeBlancObject> = vec![];
    let mut v3: Vec<LeBlancObject> = vec![];
    let count = 1000000;

    //let mut heap: Heap<LeBlancObject> = Heap::new(count);
    println!("Starting test");
    //let bump = Bump::with_capacity(count);
    for i in 0..run_amount {
        let now = Instant::now();
        for i in 0..count {
            v.push(LeBlancObject::null());
            //v3.push(LeBlancObject::null());
            //v2.push(bump.alloc(LeBlancObject::null()));
        }
        drop(v);
        drop(v2);
        drop(v3);
        v = vec![];
        v2 = vec![];
        v3 = vec![];
        let elapsed = now.elapsed();
        println!("{}", elapsed.as_secs_f64());
    }

    /*println!("Vanilla");
    for i in 0..count {
        v3.push(LeBlancObject::null());
    }*/

    /*println!("Bumpalo");
    let bump = bumpalo::Bump::new();
    for i in 0..count {
        v2.push(bump.alloc(LeBlancObject::null()));
    }*/

    LeBlancObject::null()._clone()

}

fn bump_test(count: usize) {
    let mut v2: Vec<&mut LeBlancObject> = vec![];
    let mut bump = bumpalo::Bump::new();
    for i in 0..count {
        //v.push(heap.alloc_with(LeBlancObject::null));
        //v3.push(LeBlancObject::null());
        //v2.push(bump.alloc(LeBlancObject::null()));
    }
}






pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}