#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![no_main]
#![no_std]

mod x86;
mod crt;

#[no_mangle]
#[start]
pub extern fn rusternel_main() {
    crt::putc(b'H');
    crt::putc(b'e');
    crt::putc(b'l');
    crt::putc(b'l');
    crt::putc(b'o');
    crt::putc(b',');
    crt::putc(b'r');
    crt::putc(b'u');
    crt::putc(b's');
    crt::putc(b't');
    crt::putc(b'e');
    crt::putc(b'r');
    crt::putc(b'n');
    crt::putc(b'e');
    crt::putc(b'l');
    crt::putc(b'!');
    loop {
        x86::hlt()
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {
        x86::hlt()
    }
}
