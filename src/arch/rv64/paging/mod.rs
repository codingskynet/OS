mod addr;
mod page_table;

use core::arch::asm;

use crate::arch::rv64::paging::addr::HIGHER_HALF_OFFSET;
use crate::arch::rv64::paging::page_table::{PageTable, PteFlags, SATP_MODE_SV39, ppn, vpn2};
use crate::util::r#const::G;

const DIRECT_MAP_SIZE: usize = 256 * G;

// Temporary Sv39 root page table using 1GiB leaf mappings.
static mut TEMP: PageTable = PageTable::new();

pub unsafe fn enable_mmu() {
    unsafe {
        // Build a temporary bootstrap address space with two 1GiB-leaf windows:
        //
        // Identity mapping for the MMU-enable trampoline:
        // VA 0x0000_0000_0000_0000 .. 0x0000_003f_ffff_ffff
        // PA 0x0000_0000_0000_0000 .. 0x0000_003f_ffff_ffff

        // Higher-half direct mapping:
        // VA 0xffff_ffc0_0000_0000 .. 0xffff_ffff_ffff_ffff
        // PA 0x0000_0000_0000_0000 .. 0x0000_003f_ffff_ffff
        //
        // The identity half is needed for the instructions immediately after
        // writing satp. The CPU keeps fetching at the current low virtual PC
        // until we explicitly jump to the higher-half alias below.
        // Use a raw pointer so this early boot code does not create Rust
        // references to a `static mut`.
        let temp = &raw mut TEMP;

        let mut address = 0;
        let flag =
            PteFlags::V | PteFlags::R | PteFlags::W | PteFlags::X | PteFlags::A | PteFlags::D;
        while address < DIRECT_MAP_SIZE {
            (*temp)
                .entry(vpn2(address))
                .mut_address(address)
                .mut_flags(flag);
            (*temp)
                .entry(vpn2(address + HIGHER_HALF_OFFSET))
                .mut_address(address)
                .mut_flags(flag);
            address += G;
        }

        let satp = SATP_MODE_SV39 | ppn(temp as *const PageTable as usize);
        asm!("sfence.vma zero, zero", options(nostack, preserves_flags));
        asm!("csrw satp, {}", in(reg) satp, options(nostack, preserves_flags));
        asm!("sfence.vma zero, zero", options(nostack, preserves_flags));

        // Switch the current instruction stream from the low identity alias to
        // the higher-half alias. The label address produced by `lla` is still
        // the link/load address, so add HIGHER_HALF_OFFSET and jump there.
        //
        // After the jump, execution resumes at label 2 through the higher-half
        // mapping. The temporary identity map intentionally remains installed
        // until a final kernel page table can remove it.
        //
        // The stack pointer is not adjusted here; it still uses the low
        // identity alias until a later boot step moves the stack to its
        // higher-half address.
        asm!(
            "lla  {target}, 2f",
            "add  {target}, {target}, {offset}",
            "jr   {target}",
            "2:",
            target = out(reg) _,
            offset = in(reg) HIGHER_HALF_OFFSET,
            options(nostack),
        );
    }
}
