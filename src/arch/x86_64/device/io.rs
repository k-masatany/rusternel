#[inline(always)]
pub unsafe fn hlt() {
    asm!("hlt");
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("inb %dx,%al" : "={al}" (data) : "{dx}" (port));
    data
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn inw(port: u16) -> u16 {
    let data: u16;
    asm!("inw %dx,%ax" : "={ax}" (data) : "{dx}" (port));
    data
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn inl(port: u16) -> u32 {
    let data: u32;
    asm!("inl %dx,%eax" : "={eax}" (data) : "{dx}" (port));
    data
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb %al, %dx" : : "{al}"(val), "{dx}"(port));
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("outw %ax, %dx" : : "{ax}"(val), "{dx}"(port));
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn outl(port: u16, val: u32) {
    asm!("outl %eax, %dx" : : "{eax}"(val), "{dx}"(port));
}
