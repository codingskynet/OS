use crate::arch::consts::*;
use crate::mm::region::Region;

pub fn kernel() -> Region {
    unsafe { Region::from_raw(&_kernel_start, &_kernel_end) }
}

pub fn rx() -> Region {
    unsafe { Region::from_raw(&_text_start, &_rodata_end) }
}

pub fn rw() -> Region {
    unsafe { Region::from_raw(&_data_start, &_bss_end) }
}

pub fn text() -> Region {
    unsafe { Region::from_raw(&_text_start, &_text_end) }
}

pub fn rodata() -> Region {
    unsafe { Region::from_raw(&_rodata_start, &_rodata_end) }
}

pub fn data() -> Region {
    unsafe { Region::from_raw(&_data_start, &_data_end) }
}

pub fn bss() -> Region {
    unsafe { Region::from_raw(&_bss_start, &_bss_end) }
}
