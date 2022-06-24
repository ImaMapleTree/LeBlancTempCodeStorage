use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::leblanc_object::{LeBlancObject, Reflect};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::derived::iterator_type::LeblancIterator;
use crate::leblanc::core::native_types::derived::list_type::LeblancList;

pub fn _internal_iterator_next(_self: Rc<RefCell<LeBlancObject>>, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let iterator = _self.reflect().downcast_mut_unchecked::<LeblancIterator>();
    return iterator.next().to_mutex()
}

pub fn _internal_iterator_to_list_(_self: Rc<RefCell<LeBlancObject>>, arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let iterator = _self.reflect().downcast_mut_unchecked::<LeblancIterator>();

    let mut leblanc_list = LeblancList::empty();
    while iterator.has_next() {
        leblanc_list.internal_vec.push(iterator.next().to_mutex())
    }

    return leblanc_list.create_mutex()
}