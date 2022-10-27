use crate::leblanc::configuration::HDEF_MB;
use crate::leblanc::core::leblanc_object::LeBlancObject;
use crate::leblanc::rustblanc::better_static::BetterStatic;
use crate::leblanc::rustblanc::memory::heap::{HeapRef, TypedHeap, WildHeap};
use crate::leblanc::rustblanc::types::LBObject;

pub static mut HEAP: BetterStatic<TypedHeap<LeBlancObject>> = BetterStatic::new(|| TypedHeap::new_bytes(Box::new(HDEF_MB)));
//pub static mut WILD_HEAP: BetterStatic<WildHeap> = BetterStatic::new(|| WildHeap::new(Box::new((HDEF_MB * 1)/4)));

#[inline(always)]
pub fn heap() -> &'static mut BetterStatic<TypedHeap<LeBlancObject>> {
    unsafe { &mut HEAP }
}


#[derive(Debug)]
pub struct LazyHeap {
    object_heap: TypedHeap<LeBlancObject>,
    data_heap: WildHeap
}

impl LazyHeap {
    #[inline(always)]
    pub fn alloc(&mut self, obj: LeBlancObject) -> LBObject {
        self.object_heap.alloc(obj)
    }

    #[inline(always)]
    pub fn alloc_with<F>(&mut self, f: F) -> LBObject
    where F: FnOnce() -> LeBlancObject {
        self.object_heap.alloc_with(f)
    }

    #[inline(always)]
    pub fn alloc_any<T: Send>(&mut self, obj: T) -> HeapRef<'_, T>
    where T: 'static {
        self.data_heap.alloc(obj)
    }

    #[inline(always)]
    pub fn alloc_any_with<T: Send, F>(&mut self, f: F) -> HeapRef<'_, T>
        where
            T: 'static,
            F: FnOnce() -> T
    {
        self.data_heap.alloc_with(f)
    }
}

impl Default for LazyHeap {
    fn default() -> Self {
        LazyHeap {
            object_heap: TypedHeap::new_bytes(Box::new((HDEF_MB * 3)/4)),
            data_heap: WildHeap::new(Box::new((HDEF_MB * 1)/4))
        }
    }
}