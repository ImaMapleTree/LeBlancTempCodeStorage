use std::collections::HashMap;
use crate::leblanc::compiler::parser::parse_structs::{FunctionType, IdentStore, ScopeSet};

pub fn sort_functions(type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>) {
    let mut vec = type_map.iter_mut().flat_map(|(string, map)| map.iter_mut().filter_map(|(ident, scope)| {
        if let IdentStore::Function(string, types, func_type) = ident {
            Some((func_type, scope))
        } else {
            None
        }
    }).collect::<Vec<(&FunctionType, &mut ScopeSet)>>()).collect::<Vec<(&FunctionType, &mut ScopeSet)>>();

    let mut id = 0;
    let mut tracked: Vec<&mut ScopeSet> = vec![];

    let mut i = 0;
    while i < vec.len() {
        let value = vec.get(i).unwrap();
        let func_id = value.1.get_first_id().unwrap();
        if func_id > id { id = func_id }
        if *value.0 == FunctionType::LeBlanc {
            tracked.push(vec.remove(i).1);
        } else {
            i += 1;
        }
    }
    
    vec.reverse();
    for linked in vec {
        tracked.iter_mut().filter(|v| {
            v.get_first_id().unwrap() > linked.1.get_first_id().unwrap()
        }).for_each(|v| {
            let val = v.get_first_id().unwrap();
            v.set_first_id(val-1);
        });
        linked.1.set_first_id(id);
        id -= 1;
    }



}