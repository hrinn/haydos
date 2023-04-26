use crate::arch::{self, InterruptSupport};

pub fn init() {
    arch::Target::init_interrupts();
}

fn breakpoint_handler() {
    
}