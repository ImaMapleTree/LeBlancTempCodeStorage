use std::alloc::{alloc, dealloc, Layout, realloc};
use std::ptr;

#[derive(Debug)]
pub struct LowLevelList<T> {
   ptr: *mut T,
   size: usize,
   max: usize,
}

impl<T> Default for LowLevelList<T> {
   fn default() -> Self {
      LowLevelList::new(16)
   }
}

impl<T> LowLevelList<T> {
   pub fn new(size: usize) -> LowLevelList<T> {
      let array_layout = Layout::array::<T>(size).expect("Could not allocate array");
      unsafe {
         let struct_layout = Layout::new::<LowLevelList<T>>();
         let struc = alloc(struct_layout) as *mut LowLevelList<T>;
         let ptr = alloc(array_layout) as *mut T;
         (*struc).ptr = ptr;
         (*struc).size = 0;
         (*struc).max = size;
         *Box::from_raw(struc)
      }
   }

   pub fn push(&mut self, item: T) {
      if self.size < self.max {
         println!("Push no double");
         unsafe {
            (*self.ptr.add(self.size)) = item;
            self.size += 1;
         }
      } else {
         println!("Push and double");
         unsafe {
            let array_layout = Layout::array::<T>(self.max).expect("Could not allocate array");
            self.max *= 2;
            let new_array = realloc(self.ptr as *mut u8, array_layout, self.max) as *mut T;
            self.ptr = new_array;
         }
      }
   }

   pub fn pop(&mut self) -> Option<T> {
      println!("Pop: {}", self.size);
      if self.size == 0 {
         println!("Pop Done");
         None
      } else {
         unsafe {
            let item = ptr::read(self.ptr.add(self.size));
            self.size -= 1;
            println!("Pop Done");
            Some(item)
         }
      }
   }
}

impl<T> Drop for LowLevelList<T> {
   fn drop(&mut self) {
      println!("List drop");
      let array_layout = Layout::array::<T>(self.max).expect("Could not allocate array");
      unsafe { dealloc(self.ptr as *mut u8, array_layout) }
   }
}