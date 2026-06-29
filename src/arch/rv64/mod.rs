//! RISC-V 64-bit architecture support.
//!
//! *Boot* — low-level assembly entry (`_start`), BSS zeroing, stack setup.
//! *Trap* —   (TODO) trap vector, exception / interrupt handling.
//! *Paging* — (TODO) page table management (satp).
//! *Timer* —  (TODO) RISC-V timer (mtime/mtimecmp).
//! *Context* — (TODO) context-switch assembly (`__switch`).

mod paging;

use core::arch::global_asm;

use crate::boot::{BootData, BootInfo};
use crate::dev::dt::Fdt;

global_asm!(include_str!("boot.s"));

/// Rust entry point, called from boot.s.
///
/// # Safety
///
/// This function is called directly from assembly (`boot.s`) before the Rust
/// runtime is initialized.  The caller must guarantee:
///
/// * A valid stack pointer (`sp`) has been set up.
/// * The BSS section has been zeroed.
/// * This function is only entered once on the boot hart (hart 0).
#[unsafe(no_mangle)]
pub unsafe fn _start_rust(hart_id: usize, dtb_ptr: usize) -> ! {
    unsafe {
        paging::enable_mmu();
    }

    let boot_info = BootInfo {
        boot_cpu_id: hart_id,
        boot_data: BootData::DeviceTree(Fdt::new(dtb_ptr)),
    };

    unsafe { crate::boot::kernel_boot(boot_info) }
}
