//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Setup Container

extern crate alloc;
use alloc::boxed::Box;

extern crate violet;

use violet::container::create_container;
use violet::system::config::{SystemConfig, ContainerConfig};
use violet::kernel::syscall::vsi::create_task;

use violet::resource::*;

/* Device Driver */
use violet::driver::board::sifive_u::clint_timer::ClintTimer;
use violet::driver::board::sifive_u::plic::Plic;
use violet::driver::board::sifive_u::uart::Uart;

/* MMIO */
static UART_BASE: usize = 0x1000_0000;
static CLINT_TIMER_BASE: usize = 0x0200_0000;
static PLIC_BASE: usize = 0x0C00_0000;

use super::boot_linux;

use violet::app_init;
app_init!(setup);

pub const NUM_OF_CONTAINERS: usize = 1;
pub const NUM_OF_CPUS: usize = 2;
pub const STACK_SIZE: usize = 0x4000;

pub const CORE0_SHIFT: usize = 0;
pub const CORE1_SHIFT: usize = 1;

#[used(linker)]
#[link_section = ".system_config.start"]
pub static SYSTEM_CONFIG: SystemConfig<NUM_OF_CONTAINERS> = SystemConfig {
    num_of_cpus: NUM_OF_CPUS,
    num_of_containers: 1,
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

fn setup() {
    create_container();

    init_environment();
    init_kernel();
    init_task();
}

fn init_environment() {

    let resources = get_mut_resources();

    match resources.register(Resource::Serial(Box::new(Uart::new(UART_BASE)))) {
        Ok(_) => (),
        Err(e) => panic!("Failed to register Serial: {:?}", e),
    }
    
    match resources.register(Resource::Intc(Box::new(Plic::new(PLIC_BASE)))) {
        Ok(_) => (),
        Err(e) => panic!("Failed to register Intc: {:?}", e),
    }

    match resources.register(Resource::Timer(Box::new(ClintTimer::new(CLINT_TIMER_BASE)))) {
        Ok(_) => (),
        Err(e) => panic!("Failed to register Timer: {:?}", e),
    }

}

fn init_kernel() {
    // todo
}

fn init_task() {
    // Boot Linux on core 1
    create_task(2, boot_linux, 1);
}