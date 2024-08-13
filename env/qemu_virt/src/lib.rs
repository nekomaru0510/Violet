//! qemu_virt environment

use violet::container::*;
use violet::resource::*;

use violet::arch::rv64::Rv64;
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::traits::TraitCpu;

/* Device Driver */
use violet::driver::board::sifive_u::clint_timer::ClintTimer;
use violet::driver::board::sifive_u::plic::Plic;
use violet::driver::board::sifive_u::uart::Uart;


/* Num of Processor cores */
pub const NUM_OF_CPUS: usize = 2;
pub const STACK_SIZE: usize = 0x4000;

/* MMIO */
static UART_BASE: usize = 0x1000_0000;
static CLINT_TIMER_BASE: usize = 0x0200_0000;
static PLIC_BASE: usize = 0x0C00_0000;

type Arch = Rv64;
type Intc = Plic;
type Timer = ClintTimer;
type Serial = Uart;

pub fn init_environment() {
    setup_container();
}

pub fn setup_container() {
    create_container();

    let resources = get_mut_resources();

    let mut cpu0 = Rv64::new(0);
    let mut cpu1 = Rv64::new(1);
    cpu0.add_hext(Hext{});
    cpu1.add_hext(Hext{});
    let result = resources.register(Resource::Cpu(Box::new(cpu0)));
    let result = resources.register(Resource::Cpu(Box::new(cpu1)));

    let result = resources.register(Resource::Serial(Box::new(Uart::new(UART_BASE))));
    let result = resources.register(Resource::Intc(Box::new(Plic::new(PLIC_BASE))));
    let result = resources.register(Resource::Timer(Box::new(ClintTimer::new(CLINT_TIMER_BASE))));
}

use violet::arch::rv64::get_cpuid;

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

