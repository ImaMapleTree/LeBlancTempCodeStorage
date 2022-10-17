// 3 + 2





use std::mem::size_of_val;
use crate::leblanc::core::leblanc_object::{LeBlancObject};
use crate::leblanc::core::native_types::double_type::leblanc_object_double;



fn test() -> LeBlancObject {
    println!("Size of LBO: {}", size_of_val(&leblanc_object_double(2134124.122321)));


    println!("\n---------------");
    LeBlancObject::null()

}







pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}