#![no_std]
#![no_main]

mod framebuffer;

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    framebuffer::draw_background();

    println!("HAYDOS");
    println!("v{}", 0.1);

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
