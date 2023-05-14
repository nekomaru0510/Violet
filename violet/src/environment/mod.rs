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
        },
    }
}

/*
/* CPUの登録 */
pub fn init_cpus() {
    let con = get_mut_container(0); // RootContainerの取得
    match con {
        Some(c) => {
            for i in 0..NUM_OF_CPUS {
                c.register_cpu(Rv64::new(i as u64));
            }
        }
        None => (),
    }
}

/* デバイスの登録 */
pub fn init_peripherals() {
    let con = get_mut_container(0); // RootContainerの取得
    match con {
        Some(c) => {
            c.register_serial(Uart::new(UART_BASE));
            c.register_intc(Plic::new(PLIC_BASE));
            c.register_timer(ClintTimer::new(CLINT_TIMER_BASE));
        },
        None => (),
    }
}
*/