use alloc::rc::Rc;
use std::cell::RefCell;
use crate::leblanc::core::internal::transformed_iterator::TransformedIterator;

use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData, RustDataCast};

use crate::leblanc::core::native_types::derived::iterator_type::{leblanc_object_iterator, LeblancIterator};
use crate::leblanc::core::native_types::derived::list_type::{leblanc_object_list};


pub fn _internal_iterator_next(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let iterator: &mut LeblancIterator = borrowed.data.mut_data().unwrap();
    iterator.next()
}

pub fn _internal_iterator_to_list_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    Rc::new(RefCell::new(leblanc_object_list(<LeBlancObjectData as RustDataCast<LeblancIterator>>::mut_data(&mut _self.borrow_mut().data).unwrap().to_list())))
}

pub fn _internal_iterator_filter_(_self: Rc<RefCell<LeBlancObject>>, _arguments: &mut [Rc<RefCell<LeBlancObject>>]) -> Rc<RefCell<LeBlancObject>> {
    let mut borrowed = _self.borrow_mut();
    let iterator: &mut LeblancIterator = borrowed.data.mut_data().unwrap();

    match iterator.transformed() {
        Some(trans_iter) => {
            trans_iter.filter(_arguments[0].borrow().data.get_inner_method().unwrap().leblanc_handle.borrow().clone());
            drop(borrowed);
            _self
        },
        None => {
            let mut new_iter = TransformedIterator::new(iterator.iterator.clone());
            new_iter.filter(_arguments[0].borrow().data.get_inner_method().unwrap().leblanc_handle.borrow().clone());
            leblanc_object_iterator(Box::new(new_iter)).to_mutex()
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
            trans_iter.map(_arguments[0].borrow().data.get_inner_method().unwrap().leblanc_handle.borrow().clone());
            drop(borrowed);
            _self
        },
        None => {
            let mut new_iter = TransformedIterator::new(iterator.iterator.clone());
            new_iter.map(_arguments[0].borrow().data.get_inner_method().unwrap().leblanc_handle.borrow().clone());
            leblanc_object_iterator(Box::new(new_iter)).to_mutex()
        }
    }
}