#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![no_main]
#![no_std]

pub mod arch;
pub use arch::*;

pub mod devices;

#[no_mangle]
#[start]
pub extern fn rusternel_main() {
    devices::crt::puts("Hello,rusternel!\r\n");
    unsafe {
        x86_64::gdt::init_gdtidt();
        loop {
            x86_64::device::io::hlt()
        }
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    unsafe {
        loop {
            x86_64::device::io::hlt()
        }
    }
}
