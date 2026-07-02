use core::cmp::{max, min};
use core::num::NonZeroUsize;
use core::ops::BitAnd;

use crate::mm::addr::{Pa, Va};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Region {
    pub start: Pa,
    pub end: Pa,
}

impl Region {
    pub fn new(start: Pa, end: Pa) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    pub fn from_raw(start: *const u8, end: *const u8) -> Self {
        Self {
            start: Va::new(start.addr()).into_pa(),
            end: Va::new(end.addr()).into_pa(),
        }
    }

    pub fn from_size(addr: Pa, size: NonZeroUsize) -> Option<Self> {
        let end = addr.checked_offset(size.into())?;
        Some(Region { start: addr, end })
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn contains(&self, addr: Pa) -> bool {
        self.start <= addr && addr < self.end
    }

    pub fn intersection(&self, other: Region) -> Option<Self> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);

        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }
}

impl BitAnd for Region {
    type Output = Option<Region>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}
