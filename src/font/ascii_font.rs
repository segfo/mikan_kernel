use crate::font::ascii_font_data::ASCII_FONT_DATA;
use common::hardware::FrameBufferConfig;
pub fn write_str(
    mut fb: FrameBufferConfig,
    s: &str,
    x_base: usize,
    y_base: usize,
    color: &[u8; 3],
) -> (usize, usize) {
    let mut off: usize = 0;
    let mut x = x_base;
    let mut y = y_base;
    for c in s.chars() {
        let (x0, y0) = write_char(fb, c, x, y, color);
        x = x0;
        y = y0;
        off += 1;
    }
    (x, y)
}

pub fn write_char(
    mut fb: FrameBufferConfig,
    ascii: char,
    x_base: usize,
    y_base: usize,
    color: &[u8; 3],
) -> (usize, usize) {
    if (!ascii.is_ascii()) {
        return (x_base, y_base);
    }
    let mut code = [0u8; 1];
    ascii.encode_utf8(&mut code);
    let mut x = x_base;
    let mut y = y_base;
    let code = code[0] as usize;
    match (code) {
        0x0a => {
            return (0, y + 16);
        }
        _ => {}
    }
    let font = ASCII_FONT_DATA[code];
    for h in 0..16 {
        for w in 0..8 {
            if font[h] & (1 << (7 - w)) != 0 {
                fb.write_pixel(w + x, h + y, *color);
            }
        }
    }
    (x + 8, y)
}
