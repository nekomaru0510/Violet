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

pub fn init_environment() {
    setup_container();
}

pub fn setup_container() {
    create_container();

    let resources = get_mut_resources();

    let cpu0 = Rv64::new(0);
    let cpu1 = Rv64::new(1);
    //cpu0.add_hext(Hext{});
    //cpu1.add_hext(Hext{});
    let result = resources.register(Resource::Cpu(Box::new(cpu0)));
    let result = resources.register(Resource::Cpu(Box::new(cpu1)));

    let result = resources.register(Resource::Serial(Box::new(Uart::new(UART_BASE))));
    let result = resources.register(Resource::Intc(Box::new(Plic::new(PLIC_BASE))));
    let result = resources.register(Resource::Timer(Box::new(ClintTimer::new(CLINT_TIMER_BASE))));
}

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

extern crate alloc;
use alloc::boxed::Box;

//use crate::driver::arch::rv64::Rv64;
#[test_case]
fn test_cpuget() -> Result<(), &'static str> {
    create_container();
    /*
    let con = get_mut_container(0);
    match con {
        Some(c) => c.register_cpu(Rv64::new(0)),
        None => (),
    }

    cpu_mut().wakeup();
    cpu().wakeup();
    cpu().set_default_vector();
    cpu_mut().set_default_vector();
    */
    //cpu_mut2::<Rv64>().set_default_vector();

    Ok(())
}
