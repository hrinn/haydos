#![no_std]
#![no_main]

mod framebuffer;

use core::arch::asm;
use core::panic::PanicInfo;

use framebuffer::Color;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    framebuffer::setup_terminal();

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
