pub mod addr;
pub mod buddy;
pub mod page;
pub mod region;
pub mod slab;

use core::alloc::{GlobalAlloc, Layout};

use crate::mm::buddy::BuddyAllocator;
use crate::mm::page::PageMetaMap;
use crate::mm::slab::SlabAllocator;
use crate::sync::SpinLock;

pub static PAGE_META_MAP: SpinLock<PageMetaMap> = SpinLock::new(PageMetaMap::empty());

pub static BUDDY: SpinLock<BuddyAllocator> = SpinLock::new(BuddyAllocator::empty());

#[global_allocator]
pub static GLOBAL: Allocator = Allocator::new();

pub struct Allocator {
    // TODO: All slab allocators must be per core
    slabs: [SlabAllocator; 8],
}

impl Allocator {
    const fn new() -> Self {
        Self {
            slabs: [
                SlabAllocator::new(32),
                SlabAllocator::new(64),
                SlabAllocator::new(128),
                SlabAllocator::new(256),
                SlabAllocator::new(512),
                SlabAllocator::new(1024),
                SlabAllocator::new(2048),
                SlabAllocator::new(4096),
            ],
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
