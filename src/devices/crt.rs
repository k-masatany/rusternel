const CRTC_COL: u16     = 25;
const CRTC_ROW: u16     = 80;
const CRTC_ADDR: u16    = 0x3d4;
const CRTC_DATA: u16    = 0x3d5;
const CRTC_CURSOR_H: u8 = 0x0E;
const CRTC_CURSOR_L: u8 = 0x0F;

use arch::x86_64::device::io;

// カーソル位置指定
pub unsafe fn set_cursor(pos: u16) {
    io::outb(CRTC_ADDR, CRTC_CURSOR_H);
    io::outb(CRTC_DATA, (pos >> 8) as u8);
    io::outb(CRTC_ADDR, CRTC_CURSOR_L);
    io::outb(CRTC_DATA, pos as u8);
}
// カーソル位置取得
pub unsafe fn get_cursor() -> u16 {
    io::outb(CRTC_ADDR, CRTC_CURSOR_H);
    let ph = io::inb(CRTC_DATA);
    io::outb(CRTC_ADDR, CRTC_CURSOR_L);
    let pl = io::inb(CRTC_DATA);
    (ph as u16) << 8 | (pl as u16)
}

// 文字表示
pub unsafe fn putc(c: u8) {
    let pos = get_cursor();

    match c {
        // BS:一文字戻って表示クリア（Nullを書く）
        0x08 => {
            set_cursor(pos-1);
            let mut vram = 0xb8000 + (get_cursor() * 2) as i32;
            *(vram as *mut u16) = 0x0f00;
        },
        // LF:改行（タスク：スクロール処理の実装）
        0x0A => set_cursor(pos + CRTC_ROW),
        // CR:行頭へ
        0x0D => set_cursor(pos - (pos % CRTC_ROW)),
        // 通常文字なら通常表示する
        _ => {
            let mut vram = 0xb8000 + (pos * 2) as i32;
            *(vram as *mut u16) = 0x0f00 | (c as u16);
            set_cursor(pos+1);  // 次の位置へカーソルを移動
        }
    }
}

// 文字列表示
pub fn puts(s: &str) {
    for c in s.as_bytes() {
        unsafe {
            putc(*c);
        }
    }
}
