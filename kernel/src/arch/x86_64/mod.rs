use core::arch::asm;
use crate::arch::KernelSupport;

pub struct X86_64;

impl KernelSupport for X86_64 {
    fn hcf() -> ! {
        unsafe {
            asm!("cli");
            loop {
                asm!("hlt");
            }
        }
    }
}