use core::arch::asm;
use crate::arch::KernelSupport;

pub struct RISCV;

impl KernelSupport for RISCV {
    fn hcf() -> ! {
        unsafe {
            asm!("cli");
            loop {
                asm!("hlt");
            }
        }
    }
}