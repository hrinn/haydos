#![no_std]
#![no_main]

mod framebuffer;

use core::arch::asm;
use core::panic::PanicInfo;

use limine::LimineBootInfoRequest;

static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    framebuffer::draw_background();

    println!("HAYDOS");
    println!("v{}", 0.1);

    let bootloader_info = BOOTLOADER_INFO
        .get_response()
        .get()
        .expect("limine: bootloader info response should not be empty");

    println!(
        "bootloader: {} v{}",
        bootloader_info.name.to_str().unwrap().to_str().unwrap(),
        bootloader_info.version.to_str().unwrap().to_str().unwrap()
    );

    hcf();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
