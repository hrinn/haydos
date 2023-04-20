#![no_std]
#![no_main]

mod framebuffer;

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    framebuffer::draw_background();

    framebuffer::putc('h');
    framebuffer::putc('a');
    framebuffer::putc('y');
    framebuffer::putc('d');
    framebuffer::putc('o');
    framebuffer::putc('s');
    framebuffer::putc('!');
    framebuffer::putc('\n');
    framebuffer::putc('W');
    framebuffer::putc('\n');

    for _ in 0..200 {
        framebuffer::putc('.');
    }

    hcf();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
