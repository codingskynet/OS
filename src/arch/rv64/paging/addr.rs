pub const HIGHER_HALF_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[repr(transparent)]
pub struct PA(usize);

#[repr(transparent)]
pub struct VA(usize);
