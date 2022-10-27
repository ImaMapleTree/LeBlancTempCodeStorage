



use std::time::Instant;
use crate::leblanc::configuration::HDEF_MB;


use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::rustblanc::memory::byte::Byte;


use crate::leblanc::rustblanc::memory::heap::{HeapRef, WildHeap};
use crate::leblanc::rustblanc::memory::kilobyte::Kilobyte;
use crate::leblanc::rustblanc::memory::MemoryFootprint;

#[derive(Debug, Default)]
pub struct Test {
    word: String
}

#[derive(Debug, Default)]
pub struct Other {
    word: String
}

#[derive(Debug, Default)]
pub struct SameSizeStruct {
    a: usize,
    b: usize,
    c: usize
}

#[derive(Debug, Default)]
pub struct BeegStruct {
    a: SameSizeStruct,
    b: Other,
    c: Test
}

fn test() -> LeBlancObject {
    println!("Size of LBO: {}", LeBlancObject::default().mem_size());
    println!("LB Data: {}", LeBlancObjectData::Null.mem_size());

    LeBlancObject::null()._clone()
}


















fn test2() -> LeBlancObject {
    let run_amount = 100;
    let mut v: Vec<HeapRef<LeBlancObject>> = vec![];
    let mut v2: Vec<&mut LeBlancObject> = vec![];
    let mut v3: Vec<LeBlancObject> = vec![];
    let count = 1000000;

    //let mut heap: Heap<LeBlancObject> = Heap::new(count);
    println!("Starting test");
    //let bump = Bump::with_capacity(count);
    for _i in 0..run_amount {
        let now = Instant::now();
        for _i in 0..count {
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
    let _v2: Vec<&mut LeBlancObject> = vec![];
    let _bump = bumpalo::Bump::new();
    for _i in 0..count {
        //v.push(heap.alloc_with(LeBlancObject::null));
        //v3.push(LeBlancObject::null());
        //v2.push(bump.alloc(LeBlancObject::null()));
    }
}






pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}