const CRTC_ADDR: u16    = 0x3d4;
const CRTC_DATA: u16    = 0x3d5;
const CRTC_CURSOR_H: u8 = 0x0E;
const CRTC_CURSOR_L: u8 = 0x0F;

use x86;

// カーソル位置指定
pub fn set_cursor(pos: u16) {
    x86::outb(CRTC_ADDR, CRTC_CURSOR_H);
    x86::outb(CRTC_DATA, (pos >> 8) as u8);
    x86::outb(CRTC_ADDR, CRTC_CURSOR_L);
    x86::outb(CRTC_DATA, pos as u8);
}
// カーソル位置取得
pub fn get_cursor() -> u16 {
    x86::outb(CRTC_ADDR, CRTC_CURSOR_H);
    let ph = x86::inb(CRTC_DATA);
    x86::outb(CRTC_ADDR, CRTC_CURSOR_L);
    let pl = x86::inb(CRTC_DATA);
    (ph as u16) << 8 | (pl as u16)
}

// 文字表示
pub fn putc(c: u8) {
    let pos = get_cursor();
    let vram = 0xb8000 + (pos * 2) as i32;

    match c {
        // CR:行頭へ
        //0x0A =>
        // LF:改行
        //0x0D => ,

        // 通常文字なら表示する
        _ => {
            unsafe { *(vram as *mut u16) = 0x0f00 | (c as u16); }
            set_cursor(pos+1);  // 次の位置へカーソルを移動
        }
    }


}

// // 文字列表示
// pub fn puts(s: &[u8]) {
//     for c in s.iter() {
//         putc(*c);
//     }
// }
