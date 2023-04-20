use lazy_static::lazy_static;
use limine::{LimineFramebuffer, LimineFramebufferRequest};
use spin::Mutex;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Color {
    White = 0xFFFFFFFF,
    Black = 0xFF000000,
    DarkViolet = 0xFF494368,
    Alabaster = 0xFFEEF0EB,
}

struct Framebuffer {
    cursor: u64,
    buffer: &'static LimineFramebuffer,
}

impl Framebuffer {
    unsafe fn putc(&mut self, c: char) {

    }
}

static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest::new(0);

lazy_static! {
    static ref FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer {
        cursor: 0,
        buffer: FRAMEBUFFER_REQUEST
            .get_response()
            .get()
            .expect("limine: No framebuffer response")
            .framebuffers()
            .first()
            .expect("No framebuffers"),
    });
}

fn fill_screen(color: Color) {
    let framebuffer = FRAMEBUFFER.lock();

    for i in (0..((framebuffer.buffer.pitch * framebuffer.buffer.height) as usize)).step_by(4) {
        unsafe {
            *(framebuffer
                .buffer
                .address
                .as_ptr()
                .unwrap()
                .offset(i as isize) as *mut u32) = color as u32;
        }
    }
}

fn draw_border(color: Color) {
    let framebuffer = FRAMEBUFFER.lock();

    // Top line
    for i in (64..(framebuffer.buffer.pitch - 64) as usize).step_by(4) {
        for j in 0..4 {
            let offset = i + (framebuffer.buffer.pitch as usize) * (16 + j);
            
            unsafe {
                *(framebuffer
                    .buffer
                    .address
                    .as_ptr()
                    .unwrap()
                    .offset(offset as isize) as *mut u32) = color as u32;
            }
        }
    }

    // Bottom line
    for i in (64..(framebuffer.buffer.pitch - 64) as usize).step_by(4) {
        for j in 0..4 {
            let offset = i + (framebuffer.buffer.pitch as usize) * ((framebuffer.buffer.height as usize) - j - 16);
            
            unsafe {
                *(framebuffer
                    .buffer
                    .address
                    .as_ptr()
                    .unwrap()
                    .offset(offset as isize) as *mut u32) = color as u32;
            }
        }
    }

    // Left line
    // Bottom line
    for i in (16..(framebuffer.buffer.height - 16) as usize).step_by(4) {
        for j in 0..4 {
            let offset = (i * (framebuffer.buffer.pitch as usize)) + (16 + j);
            
            unsafe {
                *(framebuffer
                    .buffer
                    .address
                    .as_ptr()
                    .unwrap()
                    .offset(offset as isize) as *mut u32) = color as u32;
            }
        }
    }

    // Right line

}

static HELLO: &[u8] = b"Hello World!";

pub fn setup_terminal() {
    fill_screen(Color::Black);

    draw_border(Color::Alabaster);
}
