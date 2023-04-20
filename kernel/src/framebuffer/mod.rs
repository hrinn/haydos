mod font;

use core::ptr::copy;
use font::FONT;
use lazy_static::lazy_static;
use limine::{LimineFramebuffer, LimineFramebufferRequest};
use spin::Mutex;

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
                    self.write_pixel(bytes_offset.try_into().unwrap(), color);
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
            unsafe { self.write_pixel(i, self.background) }
        }
    }

    #[inline]
    unsafe fn write_pixel(&mut self, bytes_offset: usize, color: Color) {
        *(self
            .buffer
            .address
            .as_ptr()
            .unwrap()
            .offset(bytes_offset as isize) as *mut u32) = color as u32;
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
            .expect("limine: Framebuffer response should be present")
            .framebuffers()
            .first()
            .expect("limine: There should be at least 1 framebuffer"),
    });
}

pub fn draw_background() {
    let mut writer = WRITER.lock();
    writer.fill_to_end();
}

pub fn putc(c: char) {
    let mut writer = WRITER.lock();
    writer.putc(c);
}
