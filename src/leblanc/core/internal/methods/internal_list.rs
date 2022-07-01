use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::internal::internal_list_iterator::LeblancVecIterator;
use crate::leblanc::core::leblanc_object::{LeBlancObject, Reflect, RustDataCast};
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator};
use crate::leblanc::core::native_types::derived::list_type::LeblancList;



pub fn _internal_list_append_(_self: Rc<RefCell<LeBlancObject>>, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    list.internal_vec.push(arguments[0].clone());
    LeBlancObject::unsafe_null()
}

pub fn _internal_list_iterate_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let list: &mut LeblancList = borrowed.data.mut_data().unwrap();
    leblanc_object_iterator(Box::new(LeblancVecIterator::new(list.internal_vec.clone()))).to_mutex()
}