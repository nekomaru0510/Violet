//! env.rs template

//use crate::kernel::container::*;
use crate::container::*;
use crate::resource::*;

/* デバイスドライバ */
use crate::arch::rv64::Rv64;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::plic::Plic;
use crate::driver::board::sifive_u::uart::Uart;
use crate::arch::traits::TraitCpu;

/* CPUコア数 */
pub const NUM_OF_CPUS: usize = 2;
pub const STACK_SIZE: usize = 0x4000;

/* MMIO */
static UART_BASE: usize = 0x1000_0000;
static CLINT_TIMER_BASE: usize = 0x0200_0000;
static PLIC_BASE: usize = 0x0C00_0000;

type Arch = Rv64; /* CPUの型 */
type Intc = Plic;
type Timer = ClintTimer;
type Serial = Uart;

pub fn init_environment() {
    setup_container();
}

pub fn setup_container() {
    create_container();

    let resources = get_mut_resources();
    let result = resources.register(Resource::Cpu(Box::new(Rv64::new(0))));
    let result = resources.register(Resource::Cpu(Box::new(Rv64::new(1))));
    let result = resources.register(Resource::Serial(Box::new(Uart::new(UART_BASE))));
    let result = resources.register(Resource::Intc(Box::new(Plic::new(PLIC_BASE))));
    let result = resources.register(Resource::Timer(Box::new(ClintTimer::new(CLINT_TIMER_BASE))));
}

use crate::arch::rv64::get_cpuid;

pub fn cpu() -> &'static dyn TraitCpu {
    if let BorrowResource::Cpu(x) = get_resources().get(ResourceType::Cpu, get_cpuid()) {
        x.as_ref()
    } else {
        panic!("Fail to get CPU resource");
    }
}

/* CPU取得関数 */
pub fn cpu_mut() -> &'static mut dyn TraitCpu {
    if let BorrowMutResource::Cpu(x) = get_mut_resources().get_mut(ResourceType::Cpu, get_cpuid()) {
        x.as_mut()
    } else {
        panic!("Fail to get CPU resource");
    }
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
