cfg_if::cfg_if! {
    if #[cfg(target_arch="x86_64")] {
        pub mod x86_64;
        pub type Target = x86_64::X86_64;
    } else if #[cfg(target_arch="riscv")] {
        pub mod riscv;
        pub type Target = riscv::RISCV;
    } else if #[cfg(target_arch="aarch64")] {
        pub mod aarch64;
        pub type Target = aarch64::AARCH64;
    } else {
        panic!("Unsupported target architecture");
    }
}

pub trait KernelSupport {
    fn hcf() -> !;
}