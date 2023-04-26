#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod arch;
mod framebuffer;
mod interrupts;

use arch::BaseSupport;
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

    interrupts::init();

    arch::Target::hcf();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    arch::Target::hcf();
}
