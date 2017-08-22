#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![no_main]
#![no_std]

mod x86;
mod crt;
mod descriptor;

#[no_mangle]
#[start]
pub extern fn rusternel_main() {
    crt::puts("Hello,rusternel!\r\n");
    unsafe {
        descriptor::init_gdtidt();
        loop {
            x86::hlt()
        }
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    unsafe {
        loop {
            x86::hlt()
        }
    }
}
