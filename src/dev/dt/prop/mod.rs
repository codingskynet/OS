use core::ffi::CStr;

use crate::dev::dt::RegIter;

pub mod reg;

#[derive(Debug, PartialEq, Eq)]
pub struct Value<'a>(&'a [u8]);

impl<'a> Value<'a> {
    pub fn new(value: &'a [u8]) -> Self {
        Self(value)
    }

    pub fn as_slice(self) -> &'a [u8] {
        &self.0
    }

    pub fn as_str(self) -> Option<&'a str> {
        CStr::from_bytes_until_nul(self.0).ok()?.to_str().ok()
    }

    pub fn as_reg(self, address_cells: u32, size_cells: u32) -> RegIter<'a> {
        RegIter::new(self.0, address_cells, size_cells)
    }
}
