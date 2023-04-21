mod font;

use core::fmt;
use core::ptr::copy;
use font::FONT;
use lazy_static::lazy_static;
use limine::{LimineFile, LimineFramebuffer, LimineFramebufferRequest};
use spin::Mutex;

use crate::limine_module;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum Color {
    White = 0xFFFFFFFF,
    // Black = 0xFF000000,
    Purple = 0xFF541B54,
}

pub struct Writer {
    cursor: u64,
    background: Color,
    foreground: Color,
    buffer: &'static LimineFramebuffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    pub fn write_char(&mut self, c: char) {
        if !c.is_ascii() {
            panic!();
        }

        // Handle special chars
        if c == '\n' {
            self.cursor -= self.cursor % self.buffer.pitch;
            self.cursor += self.buffer.pitch * 16;
            return;
        }

        // Get offset of first char byte in the bitmap
        let bitmap_base = (((c as u8) - 0x20) as usize) * 16;

        // Write each row
        for row in 0..16 as u64 {
            let bitmap = FONT[bitmap_base + row as usize];

            // Write each bit of the row
            for col in 0..8 as u64 {
                let color = if bitmap & 0x80 >> col == 0 {
                    self.background
                } else {
                    self.foreground
                };

                let bytes_offset = self.cursor + (col * 4) + (row * self.buffer.pitch);

                unsafe {
                    self.write_pixel(bytes_offset.try_into().unwrap(), color as u32);
                }
            }
        }

        // Iterate cursor
        self.cursor += 32;

        // Check if we are at the end of the line
        if self.cursor % self.buffer.pitch == 0 {
            // Check if we have filled the screen
            if self.cursor == self.buffer.pitch * self.buffer.height {
                // Scroll screen
                unsafe {
                    copy(
                        self.buffer
                            .address
                            .as_ptr()
                            .unwrap()
                            .add((self.buffer.pitch * 16) as usize),
                        self.buffer.address.as_ptr().unwrap(),
                        self.buffer.size() - (self.buffer.pitch as usize) * 16,
                    );
                }

                // Set cursor to last line
                self.cursor = self.buffer.size() as u64 - (self.buffer.pitch) * 16;

                // Wipe last line
                self.fill_to_end();
            } else {
                // Go to beginning of next line
                self.cursor += self.buffer.pitch * 16;
            }
        }
    }

    fn fill_to_end(&mut self) {
        for i in (self.cursor as usize..self.buffer.size()).step_by(4) {
            unsafe { self.write_pixel(i, self.background as u32) }
        }
    }

    unsafe fn draw_bmp(&mut self, bmp: &LimineFile) {
        let bmp_base = bmp.base.as_ptr().unwrap();

        let img_data_offset = bmp_base.offset(0xA).read() as u32;
        let img_base = bmp_base.add(img_data_offset as usize);

        let width = bmp_base.offset(0x12).read() as u16;
        let height = bmp_base.offset(0x16).read() as u16;

        let mut fb_offset = 0;

        for b in (0..(width * height) as isize).step_by(4) {
            let mut bytes: [u8; 4] = [0xFF, 0, 0, 0];

            bytes[1] = *img_base.offset(b);
            bytes[1] = *img_base.offset(b + 1);
            bytes[1] = *img_base.offset(b + 2);

            self.write_pixel(fb_offset, u32::from_be_bytes(bytes));

            fb_offset += 4;

            if fb_offset % width as usize == 0 {
                fb_offset += self.buffer.pitch as usize - (fb_offset % self.buffer.pitch as usize);
            }
        }
    }

    #[inline]
    unsafe fn write_pixel(&mut self, bytes_offset: usize, color: u32) {
        *(self
            .buffer
            .address
            .as_ptr()
            .unwrap()
            .offset(bytes_offset as isize) as *mut u32) = color;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest::new(0);

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        cursor: 0,
        background: Color::Purple,
        foreground: Color::White,
        buffer: FRAMEBUFFER_REQUEST
            .get_response()
            .get()
            .unwrap()
            .framebuffers()
            .first()
            .unwrap(),
    });
}

pub fn draw_background() {
    let mut writer = WRITER.lock();
    writer.fill_to_end();

    // Draw bmp
    let bmp = limine_module::get_module("/hayden.bmp").expect("/hayden.bmp module not found");
    unsafe {
        writer.draw_bmp(bmp);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
