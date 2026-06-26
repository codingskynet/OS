# OS

The toy project for implementing OS

## How to Setup & Build & Run

```bash
make setup      # install dependencies including QEMU
make run        # build + create kernel.img + boot on QEMU
make build      # build only (ELF)
make image      # build + create kernel.img
make clean      # remove artifacts
```

# Reference
- https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
- https://github.com/cccriscv/mini-riscv-os
