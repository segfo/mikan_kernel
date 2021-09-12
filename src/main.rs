#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(const_fn)]
#![feature(lang_items)]
// #![feature(abi_x86_interrupt)] 
use common::{hardware::*, memory::*};
use core::fmt::Write;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86::dtables::*;
use spin::Mutex;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe {
            io::halt();
        }
    }
}
// 色を表現する構造体、RGBからHSV色空間に変換したり。
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
    // 色空間の変換メソッド
    fn hsv2rgb(&mut self, h: u8, s: u8, v: u8) {
        let h = h % 255;
        let h = h as f64 / 255.0;
        let s = s as f64 / 255.0;
        let v = v as f64 / 255.0;
        let mut r = v;
        let mut g = v;
        let mut b = v;

        let mut h = h;
        if s > 0.0 {
            h *= 6.0;
            let i = h as u32;
            let f = h - (i as f64);
            match i {
                0 => {
                    g *= 1.0 - s * (1.0 - f);
                    b *= 1.0 - s;
                }
                1 => {
                    r *= 1.0 - s * f;
                    b *= 1.0 - s;
                }
                2 => {
                    r *= 1.0 - s;
                    b *= 1.0 - s * (1.0 - f);
                }
                3 => {
                    r *= 1.0 - s;
                    g *= 1.0 - s * f;
                }
                4 => {
                    r *= 1.0 - s * (1.0 - f);
                    g *= 1.0 - s;
                }
                5 => {
                    g *= 1.0 - s;
                    b *= 1.0 - s * f;
                }
                _ => {}
            }
        }
        self.r = (r * (255.0)) as u8;
        self.g = (g * (255.0)) as u8;
        self.b = (b * (255.0)) as u8;
    }
    fn to_rgb_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

mod font;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[no_mangle]
extern "sysv64" fn kernel_main(
    mut frame_buffer: FrameBufferConfig,
    mut pmm: PageMemoryManager,
) -> ! {
    frame_buffer.init();

    let hr = frame_buffer.get_horizontal_resolution();
    let vr = frame_buffer.get_vertical_resolution();
    let mut color = Color::new();
    color.hsv2rgb((90 * 255 / 360) as u8, 0x22, 0xff);
    for y in 0..(vr * 3 / 4) {
        for x in 0..hr {
            unsafe {
                frame_buffer.write_pixel(x, y, color.to_rgb_array());
            }
        }
    }
    color.hsv2rgb(0 as u8, 0x22, 0xff);
    for y in (vr * 3 / 4)..vr {
        for x in 0..hr {
            unsafe {
                frame_buffer.write_pixel(x, y, color.to_rgb_array());
            }
        }
    }

    loop {
        unsafe {
            io::halt();
        }
    }
}
