// use core::alloc::{GlobalAlloc, Layout};

// use arrayvec::ArrayVec;

// #[global_allocator]
// static ALLOCATOR: Allocator = Allocator::new();

// struct Region {
//     start: usize,
//     end: usize,
// }

// // Simple allocator only during boot session until MMU init
// // Derive from linux memblock
// pub struct Allocator {
//     reserved: ArrayVec<Region, 128>,
//     cursor: usize,
// }

// impl Allocator {
//     const fn new() -> Self {
//         Self {
//             reserved: ArrayVec::new_const(),
//             cursor: todo!(),
//         }
//     }
// }

// unsafe impl GlobalAlloc for Allocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         todo!()
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         todo!()
//     }
// }
