pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

pub fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("inb %dx,%al" : "={al}" (data) : "{dx}" (port));
    }
    data
}

pub fn inw(port: u16) -> u16 {
    let data: u16;
    unsafe {
        asm!("inw %dx,%ax" : "={ax}" (data) : "{dx}" (port));
    }
    data
}

pub fn inl(port: u16) -> u32 {
    let data: u32;
    unsafe {
        asm!("inl %dx,%eax" : "={eax}" (data) : "{dx}" (port));
    }
    data
}

pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!("outb %al, %dx" : : "{al}"(val), "{dx}"(port));
    }
}

pub fn outw(port: u16, val: u16) {
    unsafe {
        asm!("outw %ax, %dx" : : "{ax}"(val), "{dx}"(port));
    }
}

pub fn outl(port: u16, val: u32) {
    unsafe {
        asm!("outl %eax, %dx" : : "{eax}"(val), "{dx}"(port));
    }
}
