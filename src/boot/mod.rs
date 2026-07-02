mod bump;

use crate::arch::consts::PAGE_SIZE;
use crate::boot::bump::{Alloc, BumpAllocator};
use crate::dev::dt::{Fdt, prop};
use crate::init::kernel_init;
use crate::mm::addr::Pa;
use crate::mm::page::{PageMeta, PageMetaSection};
use crate::mm::{BUDDY, PAGE_META_MAP};
use crate::util::debug::dump_page_list;
use crate::{console, println};

pub struct BootInfo {
    /// Hardware identifier of the CPU that entered the common kernel path.
    pub boot_cpu_id: usize,
    /// Platform description handed over by firmware or the bootloader.
    pub boot_data: BootData,
}

pub enum BootData {
    /// Flattened Device Tree pointer, commonly used by RISC-V and ARM systems.
    DeviceTree(Fdt),
}

/// Kernel entry point
///
/// `boot_info` describes the boot CPU and any platform data supplied by the
/// architecture-specific entry code.
///
/// # Safety
/// It must be called with a valid stack pointer and BSS already zeroed.
pub unsafe fn kernel_boot(boot_info: BootInfo) {
    unsafe {
        let metadata = match &boot_info.boot_data {
            BootData::DeviceTree(fdt) => {
                let model = fdt
                    .query()
                    .prop("model")
                    .and_then(prop::Value::as_str)
                    .unwrap_or("(unknown)");
                println!("dtb: FDT detected, model = \"{}\"", model);
                if let Err(e) = console::install_from_fdt(fdt) {
                    println!("dtb: Failed to install console: {:?}", e);
                }

                let mut allocator =
                    BumpAllocator::new(&fdt).expect("Failed to init PhysicalAllocator");
                crate::arch::init_page_table(&fdt, || {
                    allocator
                        .alloc_uninit()
                        .expect("Failed to allocate PageTable")
                });
                init_page_metadata(allocator)
            }
        };

        dump_page_list();

        BUDDY.as_mut().initialize();
        println!("{:#?}", BUDDY.as_mut());
        kernel_init();
    }
}

fn init_page_metadata(mut allocator: BumpAllocator) {
    for memory in allocator.memories_mut() {
        let region = memory.region();
        let offset = region.start.align_down(PAGE_SIZE).as_raw() / PAGE_SIZE;
        let end = region.end.align_up(PAGE_SIZE).as_raw() / PAGE_SIZE;
        let len = end - offset;
        let page_meta = memory
            .alloc_slice(len, |i| {
                PageMeta::free(Pa::new((offset + i) * PAGE_SIZE.get()))
            })
            .expect("Failed to allocate page metadata");

        // reserve outside RAM region
        for page in &mut *page_meta {
            let page_start = page.addr();
            let page_end = page_start.checked_offset(PAGE_SIZE.get()).unwrap();
            if page_start < region.start || region.end < page_end {
                page.reserve();
            }
        }

        for reserved in memory.reserved() {
            let start = reserved.start.align_down(PAGE_SIZE).as_raw() / PAGE_SIZE - offset;
            let end = reserved.end.align_up(PAGE_SIZE).as_raw() / PAGE_SIZE - offset;
            for page in &mut page_meta[start..end] {
                page.reserve();
            }
        }
        let section = PageMetaSection::new(page_meta, offset, region);
        PAGE_META_MAP.as_mut().add(section);
    }
}
