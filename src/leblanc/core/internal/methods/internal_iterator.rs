use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::internal::transformed_iterator::TransformedIterator;

use crate::leblanc::core::leblanc_object::{LeBlancObject, Reflect, RustDataCast};
use crate::leblanc::core::native_types::base_type::ToLeblanc;
use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator, LeblancIterator};
use crate::leblanc::core::native_types::derived::list_type::{LeblancList};
use crate::LeBlancType;

pub fn _internal_iterator_next(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let iterator: &mut LeblancIterator = borrowed.data.mut_data().unwrap();
    iterator.next()
}

pub fn _internal_iterator_to_list_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    println!("To list");
    let mut boxer = _self.reflect();
    let iterator = boxer.downcast_mut::<LeblancIterator>().unwrap();
    iterator.to_list().create_mutex()
}

pub fn _internal_iterator_filter_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let iterator: &mut LeblancIterator = borrowed.data.mut_data().unwrap();

    match iterator.transformed() {
        Some(trans_iter) => {
            trans_iter.filter(_arguments[0].borrow().data.get_inner_method().unwrap().clone());
            drop(borrowed);
            return _self;
        },
        None => {
            let mut new_iter = TransformedIterator::new(iterator.iterator.clone());
            new_iter.filter(_arguments[0].borrow().data.get_inner_method().unwrap().clone());
            return leblanc_object_iterator(Box::new(new_iter)).to_mutex()
        }
    }
}

pub fn _internal_iterator_reverse_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let iterator: &mut LeblancIterator = borrowed.data.mut_data().unwrap();
    iterator.reverse();
    drop(borrowed);
    _self
}

pub fn _internal_iterator_map_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let iterator: &mut LeblancIterator = borrowed.data.mut_data().unwrap();

    match iterator.transformed() {
        Some(trans_iter) => {
            trans_iter.map(_arguments[0].borrow().data.get_inner_method().unwrap().clone());
            drop(borrowed);
            return _self;
        },
        None => {
            let mut new_iter = TransformedIterator::new(iterator.iterator.clone());
            new_iter.map(_arguments[0].borrow().data.get_inner_method().unwrap().clone());
            return leblanc_object_iterator(Box::new(new_iter)).to_mutex()
        }
    }
}