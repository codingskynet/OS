use core::ffi::CStr;

use crate::dev::dt::Fdt;
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
    /// No platform description was provided, or it is not known yet.
    None,
}

/// Kernel entry point
///
/// `boot_info` describes the boot CPU and any platform data supplied by the
/// architecture-specific entry code.
///
/// # Safety
/// It must be called with a valid stack pointer and BSS already zeroed.
pub unsafe fn kernel_boot(boot_info: BootInfo) -> ! {
    if let BootData::DeviceTree(dt) = &boot_info.boot_data {
        let model = unsafe { dt.query().prop("model") }
            .and_then(|s| CStr::from_bytes_with_nul(s).ok())
            .and_then(|s| s.to_str().ok())
            .unwrap_or("(unknown)");
        println!("dtb: FDT detected, model = \"{}\"", model);
        unsafe {
            if let Err(e) = console::install_from_fdt(dt) {
                println!("dtb: Failed to install console: {:?}", e);
            }
        }
    }
    println!("hello, world");
    panic!();
}
