.DEFAULT_GOAL := build/rusternel.img

build/rusternel_main.o: src/main.rs
    $(rustc --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C opt-level=2 -C no-prepopulate-passes -o build/rusternel_main.o src/main.rs)

build/rusternel.bin: kernel.ld build/rusternel_main.o
    $(i686-unknown-linux-gnu-ld -nostdlib -Tdata=0x00310000 build/rusternel_main.o -T kernel.ld -o build/rusternel.bin)

build/rusternel.sys: lib/secondboot.bin build/rusternel.bin
    $(cat lib/secondboot.bin build/rusternel.bin > build/rusternel.sys)

build/rusternel.img: lib/ipl.bin build/rusternel.sys
    $(mformat -f 1440 -C -B lib/ipl.bin -i build/rusternel.img ::)
    $(mcopy -i build/rusternel.img build/rusternel.sys ::)
