//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Setup Container
#![no_main]
#![no_std]
#![feature(used_with_arg)]

extern crate alloc;
use alloc::boxed::Box;

extern crate violet;

use violet::container::create_container;
use violet::system::config::{SystemConfig, ContainerConfig};
use violet::kernel::syscall::vsi::create_task;

use violet::container::*;
use violet::resource::*;

use violet::arch::rv64::Rv64;
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::rv64::instruction::Instruction;
use violet::arch::traits::TraitCpu;

/* Device Driver */
use violet::driver::board::sifive_u::clint_timer::ClintTimer;
use violet::driver::board::sifive_u::plic::Plic;
use violet::driver::board::sifive_u::uart::Uart;

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

use super::boot_linux;

use violet::app_init;
app_init!(setup);

pub const NUM_OF_CONTAINERS: usize = 1;
pub const NUM_OF_CORES: usize = 2;
pub const CORE0_SHIFT: usize = 0;
pub const CORE1_SHIFT: usize = 1;

#[used(linker)]
#[link_section = ".system_config.start"]
pub static system_config: SystemConfig<NUM_OF_CONTAINERS, NUM_OF_CORES> = SystemConfig {
    num_of_cpus: NUM_OF_CORES,
    num_of_containers: 1,
    core2container: [
        1, // Core 0
        1, // Core 1
    ],
    container: [
        ContainerConfig {
            id: 1,
            num_of_cpus: 2,
            cores: 
                1 << CORE0_SHIFT | 
                1 << CORE1_SHIFT,
            bsp: 0,
        },
    ],
};

pub fn setup() {
    create_container();
    init_environment();

    // Boot Linux on core 1
    create_task(2, boot_linux, 1);
}

pub fn init_environment() {
    
    //Arch::get_mut_core().set_container_id(1);
    
    let resources = get_mut_resources();
    let result = resources.register(Resource::Cpu(Arch::get_core()));

    let result = resources.register(Resource::Serial(Box::new(Uart::new(UART_BASE))));
    let result = resources.register(Resource::Intc(Box::new(Plic::new(PLIC_BASE))));
    let result = resources.register(Resource::Timer(Box::new(ClintTimer::new(CLINT_TIMER_BASE))));
}