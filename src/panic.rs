use core::panic::PanicInfo;

use crate::printlnk;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printlnk!("kernel panic");

    if let Some(location) = info.location() {
        printlnk!(
            "  at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        printlnk!("  at <unknown>");
    }

    printlnk!("  message: {}", info.message());

    #[allow(clippy::empty_loop)]
    loop {
        core::hint::spin_loop();
    }
}
