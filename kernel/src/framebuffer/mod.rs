mod font;

use core::fmt;
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
    Purple = 0xFF22223B,
}

pub struct Writer {
    cursor: usize,
    background: Color,
    foreground: Color,
    buffer: &'static LimineFramebuffer,
}

impl Writer {
    pub fn buffer_pitch(&self) -> usize {
        self.buffer.pitch.try_into().unwrap()
    }

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
            self.new_line();
            return;
        }

        // Get offset of first char byte in the bitmap
        let bitmap_base = (((c as u8) - 0x20) as usize) * 16;

        // Write each row
        for row in 0..16 {
            let bitmap = FONT[bitmap_base + row];

            // Write each bit of the row
            for col in 0..8 {
                let color = if bitmap & 0x80 >> col == 0 {
                    self.background
                } else {
                    self.foreground
                };

                let bytes_offset = self.cursor + (col * 4) + (row * self.buffer_pitch());

                unsafe {
                    self.write_pixel(bytes_offset, color as u32);
                }
            }
        }

        // Iterate cursor
        self.cursor += 32;

        // Check if we are at the end of the line
        if self.cursor % self.buffer_pitch() == 0 {
            if self.cursor == self.buffer.size() {
                self.scroll();
            } else {
                self.new_line();
            }
        }
    }

    fn scroll(&mut self) {
        // Copy lines (1..n) to lines (0..n-1)
        unsafe {
            copy(
                self.buffer
                    .address
                    .as_ptr()
                    .unwrap()
                    .add(self.buffer_pitch() * 16),
                self.buffer.address.as_ptr().unwrap(),
                self.buffer.size() - self.buffer_pitch() * 16,
            );
        }

        // Set cursor to last line
        self.cursor = self.buffer.size() - (self.buffer_pitch()) * 16;

        // Wipe last line
        self.fill_to_end();
    }

    fn new_line(&mut self) {
        self.cursor -= self.cursor % self.buffer_pitch();
        self.cursor += self.buffer_pitch() * 16;
    }

    fn fill_to_end(&mut self) {
        for i in (self.cursor..self.buffer.size()).step_by(4) {
            unsafe { self.write_pixel(i, self.background as u32) }
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
