use core::arch::asm;
use crate::arch::KernelSupport;

pub struct AARCH64;

impl KernelSupport for AARCH64 {
    fn hcf() -> ! {
        unsafe {
            asm!("cli");
            loop {
                asm!("hlt");
            }
        }
    }
}