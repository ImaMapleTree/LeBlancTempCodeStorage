



use crate::leblanc::core::internal::transformed_iterator::TransformedIterator;

use crate::leblanc::core::native_types::derived::iterator_type::LeblancIterable;
use crate::leblanc::core::native_types::derived::list_type::LeblancList;
use crate::leblanc::rustblanc::types::{LBObject, LBObjArgs};

#[derive(Clone, Debug)]
pub struct LeblancVecIterator {
    vec: Vec<LBObject>,
    index: usize,
}

impl LeblancVecIterator {
    pub fn new(vec: Vec<LBObject>) -> LeblancVecIterator {
        LeblancVecIterator {
            vec,
            index: 0
        }
    }
}

impl LeblancIterable for LeblancVecIterator {
    fn lb_next(&mut self) -> LBObject {
        self.index += 1;
        self.vec[self.index - 1].clone()
    }

    fn has_next(&self) -> bool {
        self.index < self.vec.len()
    }

    fn reverse(&mut self) {
        self.vec.reverse()
    }

    fn to_list(&mut self) -> LeblancList {
        LeblancList::new(self.vec.clone())
    }

    fn to_rust_iter(&mut self) -> Box<dyn Iterator<Item=LBObject>> {
        Box::new(self.vec.clone().into_iter())
    }

    fn transformed(&mut self) -> Option<&mut TransformedIterator> {
        None
    }
}