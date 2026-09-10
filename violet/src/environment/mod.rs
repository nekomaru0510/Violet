#[cfg(target_device = "qemu_virt")]
pub mod qemu_virt;
#[cfg(target_device = "qemu_virt")]
pub use self::qemu_virt::*;

#[cfg(target_device = "hifive_premier_p550")]
pub mod hifive_premier_p550;
#[cfg(target_device = "hifive_premier_p550")]
pub use self::hifive_premier_p550::*;

pub const STACK_SIZE: usize = 0x10000; // 64KiB per core