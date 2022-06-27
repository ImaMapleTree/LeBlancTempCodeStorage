use alloc::rc::Rc;
use std::cell::RefCell;

use crate::leblanc::core::leblanc_object::{LeBlancObject, Reflect};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::derived::iterator_type::LeblancIterator;
use crate::leblanc::core::native_types::derived::list_type::LeblancList;

pub fn _internal_iterator_next(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut boxer = _self.reflect();
    let iterator = boxer.downcast_mut::<LeblancIterator>().unwrap();
    iterator.next().to_mutex()
}

pub fn _internal_iterator_to_list_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut boxer = _self.reflect();
    let iterator = boxer.downcast_mut::<LeblancIterator>().unwrap();
    let mut list = vec![];
    while iterator.has_next() {

        list.push(iterator.next().to_mutex())
    }
    let leblanc_list = LeblancList::new(list);

    leblanc_list.create_mutex()
}