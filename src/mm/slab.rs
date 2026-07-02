use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use crate::mm::addr::Pa;
use crate::mm::page::PageMeta;

pub struct SlabAllocator {
    size: usize,
    head: Option<NonNull<PageMeta>>,
}

impl SlabAllocator {
    pub const fn new(size: usize) -> Self {
        assert!(size.count_ones() == 1);
        Self { size, head: None }
    }
}
