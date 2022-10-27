use std::alloc::{alloc_zeroed, Layout};
use std::mem::align_of;
use std::ptr;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use parking_lot::Mutex;

const BLOCK_ALIGN: usize = align_of::<u64>();

pub(crate) type LBptr = usize;
type AtomicLBptr = AtomicUsize;

pub struct Block {
    allocations: AtomicUsize,
    pointer: AtomicUsize,
    boundary: LBptr
}

pub struct Heap {
    blocks: Mutex<Vec<Block>>,
    min_block_size: usize,
    page_size: usize
}

impl Heap {
    pub fn new(block_size: usize, page_size: usize) -> Heap {
        Heap { blocks: Default::default(), min_block_size: block_size, page_size }
    }

    pub fn alloc(&mut self, layout: Layout) -> LBptr {
        let align = layout.align().max(8);

        // Check for available block (either through deref or new allocation)
        let mut blocks = self.blocks.lock();
        while let Some(mut block) = blocks.pop() {
            let block_ptr = block.pointer.get_mut();
            let addr = round_to_multiple((*block_ptr) as usize, align);
            let next = addr.wrapping_add(layout.size());
            // Check if we're out of the block boundary
            if next < block.boundary as usize {
                block.allocations.fetch_add(1, Ordering::SeqCst);
                block.pointer.store(next as usize, Ordering::SeqCst);
                // Push block into current available allocations
                blocks.push(block);
                //println!("Existing block returns this addr: {}", addr);
                return addr as LBptr;
            }
        }
        drop(blocks);

        // No available blocks for allocation, create a new block

        let block_size = self.min_block_size.max(round_to_multiple(layout.size(), self.page_size));
        let block_ptr = alloc_block(block_size);
        //println!("Block ptr: {:?}", block_ptr);
        let block_addr =  block_ptr as usize;
        //println!("Block addr: {}", block_addr);
        //println!("And as ptr: {:?}", block_addr as *mut u8);
        let next = round_to_multiple(block_addr, align);
        //println!("Next: {}", next);
        self.blocks.lock().push(
            Block {
                allocations: AtomicUsize::new(1),
                pointer: AtomicLBptr::new(next.wrapping_add(layout.size()) as usize),
                boundary: block_addr.wrapping_add(block_size) as LBptr
            }
        );
        //println!("Old old block returns this addr: {}", next);
        next as LBptr
    }
}

fn alloc_block(size: usize) -> *mut u8 {
    unsafe { alloc_zeroed(Layout::from_size_align(size, BLOCK_ALIGN).unwrap())}
}


fn round_to_multiple(value: usize, multiple: usize) -> usize {
    (value + multiple - 1) / multiple * multiple
}


