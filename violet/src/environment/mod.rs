//! 動作環境
//pub mod qemu;

use crate::kernel::container::*;

/* デバイスドライバ */
use crate::driver::arch::rv64::Rv64;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::plic::Plic;
use crate::driver::board::sifive_u::uart::Uart;

/* CPUコア数 */
pub const NUM_OF_CPUS: usize = 2;
pub const STACK_SIZE: usize = 0x4000;

/* MMIO */
static UART_BASE: usize = 0x1000_0000;
static CLINT_TIMER_BASE: usize = 0x0200_0000;
static PLIC_BASE: usize = 0x0C00_0000;

/* CPUの型 */
type Arch = Rv64;

/* Container Table */
static mut CONTAINER_TABLE: ContainerTable<Arch> = ContainerTable::new();

pub fn init_environment() {
    setup_root_container();
}

pub fn setup_root_container() {
    if create_container() != 0 {
        panic!("Already root container exists");
    }
    let con = get_mut_container(0); // RootContainerの取得
    match con {
        Some(c) => {
            for i in 0..NUM_OF_CPUS {
                c.register_cpu(Rv64::new(i as u64));
            }
            c.register_serial(Uart::new(UART_BASE));
            c.register_intc(Plic::new(PLIC_BASE));
            c.register_timer(ClintTimer::new(CLINT_TIMER_BASE));
        }
        None => {
            panic!("Failed to create root container");
        }
    }
}

use crate::driver::arch::rv64::get_cpuid;

pub fn cpu() -> &'static Arch {
    let con = get_container(current_container_id());
    match con {
        /* [todo fix]コンテナが複数あった際に、cpuのインデックス=cpuidとならない */
        Some(c) => match c.cpu[get_cpuid()].as_ref() {
            None => {
                panic!("Nothing Cpu in Container");
            }
            Some(p) => p.as_ref(),
        },
        None => {
            panic!("Nothing Container");
        }
    }
}

/* CPU取得関数 */
pub fn cpu_mut() -> &'static mut Arch {
    let con = get_mut_container(current_container_id());
    match con {
        /* [todo fix]コンテナが複数あった際に、cpuのインデックス=cpuidとならない */
        Some(c) => match c.cpu[get_cpuid()].as_mut() {
            None => {
                panic!("Nothing Cpu in Container");
            }
            Some(p) => p.as_mut(),
        },
        None => {
            panic!("Nothing Container");
        }
    }
}

extern crate alloc;
use alloc::boxed::Box;

/* [todo delete] コンテナ関連の操作は要削除 */

pub fn create_container() -> usize {
    unsafe { CONTAINER_TABLE.add() }
}

pub fn get_mut_container(id: usize) -> Option<&'static mut Box<Container<Arch>>> {
    unsafe { CONTAINER_TABLE.get_mut(id) }
}

pub fn get_container(id: usize) -> Option<&'static Box<Container<Arch>>> {
    unsafe { CONTAINER_TABLE.get(id) }
}

pub fn current_container_id() -> usize {
    unsafe { CONTAINER_TABLE.current_id() }
}

pub fn current_container() -> Option<&'static Box<Container<Arch>>> {
    get_container(current_container_id())
}

pub fn current_mut_container() -> Option<&'static mut Box<Container<Arch>>> {
    get_mut_container(current_container_id())
}

#[cfg(test)]
use crate::driver::traits::cpu::TraitCpu;
//use crate::driver::arch::rv64::Rv64;
#[test_case]
fn test_cpuget() -> Result<(), &'static str> {
    create_container();
    let con = get_mut_container(0);
    match con {
        Some(c) => c.register_cpu(Rv64::new(0)),
        None => (),
    }

    cpu_mut().wakeup();
    cpu().wakeup();
    cpu().set_default_vector();
    cpu_mut().set_default_vector();
    //cpu_mut2::<Rv64>().set_default_vector();

    Ok(())
}
