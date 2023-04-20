use lazy_static::lazy_static;
use limine::{LimineFramebuffer, LimineFramebufferRequest};
use spin::Mutex;

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

pub fn fill_screen() {
    let framebuffer = FRAMEBUFFER.lock();

    for i in 0..framebuffer.buffer.pitch * framebuffer.buffer.height as usize {
        unsafe {
            *(framebuffer
                .buffer
                .address
                .as_ptr()
                .unwrap()
                .offset(i * 4 as isize) as *mut u32) = 0xFFFFFFFF;
        }
    }

    // for i in 0..framebuffer.buffer.height as usize {
    //     // Calculate the pixel offset using the framebuffer information we obtained above.
    //     // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
    //     let pixel_offset = i * framebuffer.buffer.pitch as usize + i * 4;

    //     // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
    //     // We can safely unwrap the result of `as_ptr()` because the framebuffer address is
    //     // guaranteed to be provided by the bootloader.
    //     unsafe {
    //         *(framebuffer
    //             .buffer
    //             .address
    //             .as_ptr()
    //             .unwrap()
    //             .offset(pixel_offset as isize) as *mut u32) = 0xFFFFFFFF;
    //     }
    // }
}
