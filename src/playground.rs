// 3 + 2


use crate::leblanc::core::interpreter::instructions::Instruction;
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::double_type::leblanc_object_double;
use crate::leblanc::core::native_types::float_type::leblanc_object_float;
use crate::leblanc::core::native_types::int128_type::leblanc_object_int128;
use crate::leblanc::core::native_types::int_type::leblanc_object_int;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::string_type::leblanc_object_string;
use std::fmt::Write;
use std::num::ParseIntError;
use crate::leblanc::rustblanc::Hexable;

fn test() -> LeBlancObject {
    let s3: f64 = 9999999999999999.4914820942841290184014820;
    let mut s = String::new();

    let hex = Instruction::BinaryMultiply.to_hex(Some(2));

    let instruct = Instruction::from_hex(hex.clone());

    println!("{} | {:?}", hex, instruct);


    //let method = a.methods.iter().filter(|method| method.tags.contains(&MethodTag::Addition)).last();



    // raise error;
    println!("-------");
    return LeBlancObject::null();

}







pub fn playground() {
    let result = test();



    println!("{:#?}", result.data);
}