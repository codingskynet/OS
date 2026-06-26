ARCH      ?= riscv64gc-unknown-none-elf
MACHINE   ?= virt
LINKER_SCRIPT := src/machine/rv64/$(MACHINE)/kernel.ld

TARGET_DIR := target/$(ARCH)/release
KERNEL_ELF := $(TARGET_DIR)/kernel
KERNEL_IMG := kernel.img

RUSTFLAGS := -C link-args=--script=$(LINKER_SCRIPT)

.PHONY: all setup build image run clean

all: build image

setup:
	@echo "==> Installing dependencies..."
	@scripts/setup.sh

build:
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc \
		--target=$(ARCH) \
		--release

image: build
	rust-objcopy --strip-all -O binary $(KERNEL_ELF) $(KERNEL_IMG)

run: image
	qemu-system-riscv64 \
		-machine $(MACHINE) \
		-nographic \
		-bios none \
		-kernel $(KERNEL_IMG)

clean:
	rm -f $(KERNEL_IMG)
	cargo clean