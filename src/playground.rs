// 3 + 2





use std::mem::size_of_val;
use crate::leblanc::core::leblanc_object::{LeBlancObject};
use crate::leblanc::core::native_types::double_type::leblanc_object_double;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::blueberry::{Blueberry, BlueberryVec};



fn test() -> LeBlancObject {


    let mut test: BlueberryVec<LeBlancObject> = BlueberryVec::from(vec![LeBlancObject::null(), LeBlancObject::error()]);

    let mut mut_ref1 = test.get(0).unwrap();
    mut_ref1.typing = LeBlancType::Short;

    println!("{:#?}", test);

    let mut mut_ref2 = test.get(0).unwrap();
    mut_ref2.typing = LeBlancType::Arch;

    println!("{:#?}", test);



    println!("\n---------------");
    LeBlancObject::null()

}







pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}