pub(crate) mod converters;
pub(crate) mod cmpnt;
pub(crate) mod generator_types;
pub(crate) mod statement;
pub(crate) mod expression;
pub(crate) mod bytecode_generator;

use std::collections::HashMap;
use crate::leblanc::compiler::generator::cmpnt::determine_component;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component};
use crate::leblanc::rustblanc::component_map::ComponentMap;


// Solution is to perhaps use dynamic programming to
// accomplish what we ant
// otherwise it could be recursive or a while loop but we'll see I have to think this out
//
// Goal:
//   Typing:
//      Function params & returns, variable

// Actually it's a waste to not do everything in this call, so this is our main call and we'll
// diverge from here
pub fn generate(components: Vec<Component>) {
    let mut func_map = ComponentMap::new();
    let mut class_map = ComponentMap::new();
    components.iter().for_each(|cmpnt| determine_component(cmpnt, &mut func_map, &mut class_map));
    println!("{:#?}", func_map);
}

