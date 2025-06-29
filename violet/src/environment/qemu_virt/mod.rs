//SPDX-License-Identifier: MIT
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com>

//! QEMU virt board/environment specific constants

#[cfg(target_arch = "riscv64")]
mod riscv64 {
    use crate::arch::rv64::Rv64;
    use crate::arch::rv64::extension::hypervisor::Hext;
    pub type Arch = Rv64;
    pub type Hyp = Hext; // [todo fix] hypervisor extension is specified in Cargo.toml
}

#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

pub const NUM_OF_CPUS: usize = 2;
