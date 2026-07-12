use crate::fs;
use crate::printlnk;

const EXPECTED: &[u8] = include_bytes!("../../../../userland/rootfs/hello.txt");

pub fn smoke() {
    printlnk!("smoke-initarfs: start");

    let mut file = fs::open("/hello.txt").expect("smoke-initarfs: failed to open /hello.txt");
    let mut buffer = [0; 64];
    let len = file
        .read(&mut buffer)
        .expect("smoke-initarfs: failed to read /hello.txt");

    assert_eq!(
        &buffer[..len],
        EXPECTED,
        "smoke-initarfs: unexpected /hello.txt contents"
    );
    assert_eq!(
        file.read(&mut buffer)
            .expect("smoke-initarfs: failed to read /hello.txt at EOF"),
        0,
        "smoke-initarfs: read at EOF returned data"
    );

    printlnk!("smoke-initarfs: done bytes={len}");
}
