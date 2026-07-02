use core::ptr::NonNull;

use arrayvec::ArrayVec;

use crate::arch::consts::PAGE_SIZE;
use crate::mm::addr::Pa;
use crate::mm::region::Region;

pub struct PageMetaMap {
    sections: ArrayVec<PageMetaSection, 4>,
}

impl PageMetaMap {
    pub const fn empty() -> Self {
        Self {
            sections: ArrayVec::new_const(),
        }
    }

    pub fn add(&mut self, section: PageMetaSection) {
        self.sections.push(section);
    }

    pub fn sections(&self) -> &[PageMetaSection] {
        &self.sections
    }

    pub fn sections_mut(&mut self) -> &mut [PageMetaSection] {
        &mut self.sections
    }

    pub fn page_meta_mut(&mut self, addr: Pa) -> Option<&mut PageMeta> {
        let page_frame = addr.as_raw() / PAGE_SIZE.get();
        for section in &mut self.sections {
            if !section.region().contains(addr) {
                continue;
            }

            let index = page_frame.checked_sub(section.offset())?;
            return section.page_metas_mut().get_mut(index);
        }

        None
    }
}

pub struct PageMetaSection {
    page_meta: &'static mut [PageMeta],
    offset: usize,
    region: Region,
}

impl PageMetaSection {
    pub fn new(page_meta: &'static mut [PageMeta], offset: usize, region: Region) -> Self {
        Self {
            page_meta,
            offset,
            region,
        }
    }

    // pub fn page_metas(&self) -> &'static [PageMeta] {
    //     self.page_meta
    // }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn page_metas(&self) -> &[PageMeta] {
        self.page_meta
    }

    pub fn page_metas_mut(&mut self) -> &mut [PageMeta] {
        self.page_meta
    }
}

pub struct PageMeta {
    addr: Pa,
    pub status: Status,
    pub order: usize,
    pub next: Option<NonNull<PageMeta>>,
}

impl PageMeta {
    pub const fn free(addr: Pa) -> Self {
        Self {
            addr,
            status: Status::Free,
            order: 0,
            next: None,
        }
    }

    pub fn addr(&self) -> Pa {
        self.addr
    }

    pub fn reserve(&mut self) {
        self.status = Status::Reserved;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum Status {
    Free,
    Reserved,
}
