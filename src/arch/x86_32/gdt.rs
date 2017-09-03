// 構造体定義
#[repr(C)]
#[repr(packed)]
struct SegmentDescriptor {
	limit_low: u16,
    base_low: u16,
	base_mid: u8,
    access_right: u8,
	limit_high: u8,
    base_high: u8,
}
#[repr(C)]
#[repr(packed)]
struct GateDescriptor {
    offset_low: u16,
    selector: u16,
	dw_count: u8,
    access_right: u8,
	offset_high: u16,
}
// 定数定義
const IDT_ADDR:	u32 	= 0x0026f800;
const LIMIT_IDT: u32 	= 0x000007ff;
const GDT_ADDR: u32 	= 0x00270000;
const LIMIT_GDT: u32 	= 0x0000ffff;
const BOTPAK_ADDR: u32 	= 0x00280000;
const LIMIT_BOTPAK: u32 = 0x0007ffff;
const AR_DATA32_RW: u16 = 0x4092;
const AR_CODE32_ER: u16 = 0x409a;
const AR_TSS32: u16 	= 0x0089;
const AR_INTGATE32: u16 = 0x008e;

pub unsafe fn init_gdtidt() {
    let gdt = GDT_ADDR;
    let idt = IDT_ADDR;

    // GDTの初期化
    for x in 0..(LIMIT_GDT / 8) {
        set_segment_descriptor(&mut*((gdt + x) as *mut SegmentDescriptor), 0, 0, 0);
    }
    set_segment_descriptor(&mut*((gdt + 1) as *mut SegmentDescriptor), 0xffffffff, 0x00000000, AR_DATA32_RW);
    set_segment_descriptor(&mut*((gdt + 2) as *mut SegmentDescriptor), LIMIT_BOTPAK, BOTPAK_ADDR, AR_CODE32_ER);
    // load_gdtr(LIMIT_GDT, ADR_GDT);

    // IDTの初期化
	for x in 0..(LIMIT_IDT / 8) {
        set_gate_descriptor(&mut*((idt + x) as *mut GateDescriptor), 0, 0, 0);
    }
    // load_idtr(LIMIT_IDT, ADR_IDT);

    // IDTの設定
    // set_gate_descriptor(idt + 0x0c, (int) asm_inthandler0c, 2 * 8, AR_INTGATE32);
    // set_gate_descriptor(idt + 0x0d, (int) asm_inthandler0d, 2 * 8, AR_INTGATE32);
    // set_gate_descriptor(idt + 0x20, (int) asm_inthandler20, 2 * 8, AR_INTGATE32);
	// set_gate_descriptor(idt + 0x21, (int) asm_inthandler21, 2 * 8, AR_INTGATE32);
	// set_gate_descriptor(idt + 0x27, (int) asm_inthandler27, 2 * 8, AR_INTGATE32);
    // set_gate_descriptor(idt + 0x2c, (int) asm_inthandler2c, 2 * 8, AR_INTGATE32);
	// set_gate_descriptor(idt + 0x40, (int) asm_exec_api,     2 * 8, AR_INTGATE32 + 0x60);
}

fn set_segment_descriptor(sd: &mut SegmentDescriptor, limit0: u32, base: u32, ar0: u16) {
	let mut ar = ar0;
	let mut limit = limit0;
    if limit > 0xfffff {
        ar = ar | 0x8000;   // G _bit = 1
        limit /= 0x1000;
    }
    sd.limit_low    = (limit & 0xffff) as u16;
    sd.base_low     = (base & 0xffff) as u16;
    sd.base_mid     = ((base >> 16) & 0xff) as u8;
    sd.access_right = (ar & 0xff) as u8;
    sd.limit_high   = (((limit >> 16) & 0x0f) as u16 | ((ar >> 8) & 0xf0)) as u8;
    sd.base_high    = ((base >> 24) & 0xff) as u8;
}

fn set_gate_descriptor(gd: &mut GateDescriptor, offset: u32, selector: u16, ar: u16) {
    gd.offset_low   = (offset & 0xffff) as u16;
    gd.selector     = selector;
    gd.dw_count     = ((ar >> 8) & 0xff) as u8;
    gd.access_right = (ar as u8) & 0xff;
    gd.offset_high  = ((offset >> 16) & 0xffff) as u16;
}
