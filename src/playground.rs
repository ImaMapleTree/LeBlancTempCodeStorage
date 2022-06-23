// 3 + 2





use crate::leblanc::core::leblanc_object::{LeBlancObject};






















fn test() -> LeBlancObject {

    println!("\n---------------");
    return LeBlancObject::null();

}







pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}