//! Runtime kernel crate.
//!
//! This crate owns the code and state that must remain valid after boot-time
//! initialization has finished. Host-side tests intentionally expose only the
//! architecture-independent modules.

#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod dev;
pub mod util;

#[cfg(not(test))]
pub mod arch;
#[cfg(not(test))]
pub mod debug;
#[cfg(not(test))]
pub mod kernel;
#[cfg(not(test))]
pub mod mm;
#[cfg(not(test))]
mod panic;
