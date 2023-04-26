use core::arch::asm;
use crate::arch::BaseSupport;

pub struct AARCH64;

impl BaseSupport for AARCH64 {
    fn hcf() -> ! {
        unsafe {
            asm!("mrs x0, cpr");
            asm!("orr x0, x0, #0x80");
            asm!("msr cpsr_cxsf, x0");
            loop {
                asm!("wfe");
            }
        }
    }
}