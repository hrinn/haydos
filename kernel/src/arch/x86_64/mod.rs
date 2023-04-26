use core::arch::asm;
use x86_64_impl::structures::idt::InterruptDescriptorTable;
use x86_64_impl::structures::idt::InterruptStackFrame;

use crate::arch::BaseSupport;
use crate::arch::InterruptSupport;

static isr_table: [Option<fn()>; 256] = [None; 256];

pub struct X86_64 {
    isr_table: [Option<fn()>; 256],
}

impl BaseSupport for X86_64 {
    fn hcf() -> ! {
        unsafe {
            asm!("cli");
            loop {
                asm!("hlt");
            }
        }
    }
}

impl InterruptSupport for X86_64 {
    fn init_interrupts(&mut self) {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_isr);
    }
}

extern "x86-interrupt" fn breakpoint_isr(stack_frame: InterruptStackFrame) {
    isr_table[2]
    .expect("No breakpoint isr set in isr_table")()
}
