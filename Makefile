.DEFAULT_GOAL := build/rusternel.img

.PHONY: clean qemu

# lto -> link time optimization: リンク時最適化
# relocation-model=static -> いろんなアドレスをstaticにするよ
# no-prepopulate-passes:
build/rusternel_main.o : src/main.rs # src/crt.rs src/x86.rs
	rustc --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj,dep-info -C lto -C opt-level=2 -C relocation-model=static -C no-prepopulate-passes -o build/rusternel_main.o src/main.rs

build/rusternel.bin : kernel.ld build/rusternel_main.o
	i686-unknown-linux-gnu-ld -nostdlib -Tdata=0x00310000 build/rusternel_main.o -T kernel.ld -o build/rusternel.bin

build/rusternel.sys : lib/secondboot.bin build/rusternel.bin
	cat lib/secondboot.bin build/rusternel.bin > build/rusternel.sys

build/rusternel.img : lib/bootsector.bin build/rusternel.sys
	mformat -f 1440 -C -B lib/bootsector.bin -i build/rusternel.img
	mcopy -i build/rusternel.img build/rusternel.sys ::

clean:
	rm -f build/*

bootloader:
	nasm -f bin -o lib/bootsector.bin lib/bootsector.asm
	nasm -f bin -o lib/secondboot.bin lib/secondboot.asm

qemu: build/rusternel.img
	qemu-system-i386 -m 32 -localtime -vga std -fda build/rusternel.img
