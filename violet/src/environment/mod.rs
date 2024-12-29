//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! env.rs template

use crate::container::*;
use crate::resource::*;

use crate::arch::rv64::Rv64;
use crate::arch::rv64::extension::hypervisor::Hext;
use crate::arch::rv64::instruction::Instruction;

/* Device Driver */
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::plic::Plic;
use crate::driver::board::sifive_u::uart::Uart;

pub const NUM_OF_CPUS: usize = 2;
pub const STACK_SIZE: usize = 0x4000;

/* MMIO */
static UART_BASE: usize = 0x1000_0000;
static CLINT_TIMER_BASE: usize = 0x0200_0000;
static PLIC_BASE: usize = 0x0C00_0000;

pub type Arch = Rv64;
pub type Hyp = Hext; // Hypervisor Extension
type Intc = Plic;
type Timer = ClintTimer;
type Serial = Uart;

pub fn shutdown() {
    let ret = Instruction::ecall(
        0x53525354,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    );
}

