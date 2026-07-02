use core::ptr::NonNull;

use crate::mm::page::PageMeta;

pub struct SlabAllocator {
    size: usize,
    head: Option<NonNull<PageMeta>>,
}

// TODO
unsafe impl Send for SlabAllocator {}

// TODO
unsafe impl Sync for SlabAllocator {}

impl SlabAllocator {
    pub const fn new(size: usize) -> Self {
        assert!(size.count_ones() == 1);
        Self { size, head: None }
    }
}
