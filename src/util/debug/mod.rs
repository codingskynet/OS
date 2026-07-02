use crate::arch::consts::PAGE_SIZE;
use crate::mm::PAGE_META_MAP;
use crate::mm::addr::Pa;
use crate::mm::page::{PageMetaSection, Status};
use crate::println;

pub fn dump_page_list() {
    let page_meta_map = PAGE_META_MAP.lock();
    let sections = page_meta_map.sections();

    if sections.is_empty() {
        println!("page metadata: empty");
        return;
    }

    let pages = sections
        .iter()
        .fold(0, |pages, section| pages + section.page_metas().len());
    println!(
        "page metadata: {} sections, {} pages",
        sections.len(),
        pages
    );

    for (index, section) in sections.iter().enumerate() {
        dump_page_section(index, section);
    }
}

fn dump_page_section(index: usize, page_meta: &PageMetaSection) {
    let pages = page_meta.page_metas();
    if pages.is_empty() {
        println!(
            "  section {}: region {}..{} (offset {}): empty",
            index,
            page_meta.region().start,
            page_meta.region().end,
            page_meta.offset(),
        );
        return;
    }

    println!(
        "  section {}: region {}..{} (offset {}, {} pages)",
        index,
        page_meta.region().start,
        page_meta.region().end,
        page_meta.offset(),
        pages.len(),
    );

    let mut start = pages[0].addr();
    let mut status = pages[0].status;
    for (_, page) in pages.iter().enumerate().skip(1) {
        if page.status != status {
            dump_page_range(start, page.addr(), status);
            start = page.addr();
            status = page.status;
        }
    }
    dump_page_range(
        start,
        pages[pages.len() - 1]
            .addr()
            .checked_offset(PAGE_SIZE.get())
            .unwrap(),
        status,
    );
}

fn dump_page_range(start: Pa, end: Pa, status: Status) {
    println!(
        "  addr {}..{}: {} ({} pages)",
        start,
        end,
        status,
        (end.as_raw() - start.as_raw()) / PAGE_SIZE.get()
    );
}
