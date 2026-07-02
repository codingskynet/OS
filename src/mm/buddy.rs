use core::fmt::Debug;
use core::mem;
use core::num::NonZeroUsize;
use core::ptr::NonNull;

use crate::arch::consts::PAGE_SIZE;
use crate::mm::PAGE_META_MAP;
use crate::mm::page::{PageMeta, Status};

pub struct BuddyAllocator {
    // nodes for 4KiB, 8KiB, 16KiB, ..., 2MiB
    heads: [Option<NonNull<PageMeta>>; 10],
}

impl BuddyAllocator {
    pub const fn empty() -> Self {
        Self { heads: [None; 10] }
    }

    pub fn initialize(&mut self) {
        fn max_order(pages: &[PageMeta], node: usize, max_order: usize) -> usize {
            let page_frame = pages[node].addr().as_raw() / PAGE_SIZE.get();
            let align_order = page_frame.trailing_zeros() as usize;
            let max_order = max_order.min(align_order);

            let mut index = 0;
            let mut best = 0;
            for order in 1..=max_order {
                while index < (1 << order) {
                    let Some(page) = pages.get(node + index) else {
                        return best;
                    };
                    if page.status != Status::Free {
                        return best;
                    }
                    index += 1;
                }
                best = order;
            }

            max_order
        }

        for section in PAGE_META_MAP.as_mut().sections_mut() {
            let pages = section.page_metas_mut();
            let mut i = 0;
            while i < pages.len() {
                if pages[i].status != Status::Free {
                    i += 1;
                    continue;
                }

                let order = max_order(pages, i, self.heads.len() - 1);
                let page = NonNull::new(&mut pages[i] as *mut _).unwrap();
                self.push(order, page);

                i += 1 << order;
            }
        }
    }

    pub fn alloc(&mut self, size: NonZeroUsize) -> Option<NonNull<PageMeta>> {
        let pages = size.get().div_ceil(PAGE_SIZE.get());
        let order = pages.checked_next_power_of_two()?.trailing_zeros() as usize;
        let order = (order < self.heads.len()).then_some(order)?;

        match self.pop(order) {
            Some(page) => Some(page),
            None => {
                self.split(order);
                self.pop(order)
            }
        }
    }

    fn pop(&mut self, order: usize) -> Option<NonNull<PageMeta>> {
        let next = unsafe { self.heads[order]?.as_ref().next };
        mem::replace(&mut self.heads[order], next)
    }

    fn push(&mut self, order: usize, mut page: NonNull<PageMeta>) {
        unsafe {
            let p = page.as_mut();
            assert!((p.addr().as_raw() / PAGE_SIZE.get()).trailing_zeros() as usize >= order);
            p.status = Status::Free;
            p.order = order;
            p.next = self.heads[order].replace(page);
        }
    }

    fn split(&mut self, order: usize) {
        let mut current_order = order;
        while current_order < self.heads.len() && self.heads[current_order].is_none() {
            current_order += 1;
        }
        if current_order == self.heads.len() {
            return;
        }

        while current_order > order {
            let Some(mut head) = self.pop(current_order) else {
                return;
            };
            current_order -= 1;
            unsafe {
                let head = head.as_mut();
                let buddy_addr = head
                    .addr()
                    .checked_offset(PAGE_SIZE.get() * (1 << current_order))
                    .unwrap();
                let buddy = PAGE_META_MAP
                    .as_mut()
                    .page_meta_mut(buddy_addr)
                    .map(NonNull::from)
                    .expect("Buddy page metadata not found");
                self.push(current_order, buddy);
                self.push(current_order, NonNull::from(head));
            }
        }
    }
}

impl Debug for BuddyAllocator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BuddyAllocator")
            .field("heads", &BuddyHeads(&self.heads))
            .finish()
    }
}

struct BuddyHeads<'a>(&'a [Option<NonNull<PageMeta>>; 10]);

impl Debug for BuddyHeads<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for (i, head) in self.0.iter().enumerate() {
            match head {
                Some(ptr) => {
                    let page = unsafe { ptr.as_ref() };
                    let len = {
                        let mut count = 1;
                        let mut cur = &page.next;
                        while let Some(n) = cur {
                            count += 1;
                            cur = unsafe { &n.as_ref().next };
                        }
                        count
                    };
                    list.entry(&format_args!("order {}: {}, len={}", i, page.addr(), len));
                }
                None => {
                    list.entry(&format_args!("order {}: None", i));
                }
            }
        }
        list.finish()
    }
}
