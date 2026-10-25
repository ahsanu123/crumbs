#[cfg(all(
    feature = "esp",
    target_os = "none",
    any(target_arch = "xtensa", target_arch = "riscv32")
))]
pub mod esp;
