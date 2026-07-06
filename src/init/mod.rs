use crate::printlnk;

pub fn kernel_init() {
    #[cfg(feature = "fuzz-allocator")]
    {
        use crate::debug::dump_page_list;
        use crate::mm::BUDDY;

        dump_page_list();
        printlnk!("{:#?}", *BUDDY.lock());
        crate::debug::fuzz::allocator::run();
        dump_page_list();
        printlnk!("{:#?}", *BUDDY.lock());
    }

    printlnk!("hello, init!");
}
