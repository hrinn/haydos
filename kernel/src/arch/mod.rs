cfg_if::cfg_if! {
    if #[cfg(target_arch="x86_64")] {
        mod x86_64;
        pub type Target = x86_64::X86_64;
    } else {
        panic!("Unsupported target architecture");
    }
}

pub trait BaseSupport {
    fn hcf() -> !;
}

pub trait InterruptSupport {
    fn init_interrupts(&mut self);
}
