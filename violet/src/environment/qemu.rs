//! QEMU(RISCV virt)

use core::mem::replace;

use crate::PERIPHERALS;

/* デバイスドライバ */
use crate::driver::arch::rv64::*;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::plic::Plic;
use crate::driver::board::sifive_u::uart::Uart;

/* ドライバ用トレイト */
//use crate::driver::traits::serial::TraitSerial;
//use crate::driver::traits::timer::TraitTimer;

use crate::environment::traits::cpu::HasCpu;
use crate::environment::traits::intc::HasIntc;
use crate::environment::traits::serial::HasSerial;
use crate::environment::traits::timer::HasTimer;

static UART_BASE: usize = 0x1000_0000;
static CLINT_TIMER_BASE: usize = 0x0200_4000;
static PLIC_BASE: usize = 0x0C00_0000;

pub fn init_peripherals() {
    unsafe {
        PERIPHERALS.cpu = Some(Rv64::new(0));
        PERIPHERALS.serial = Some(Uart::new(UART_BASE));
        PERIPHERALS.timer = Some(ClintTimer::new(CLINT_TIMER_BASE));
        PERIPHERALS.intc = Some(Plic::new(PLIC_BASE));
    }
}

#[derive(Clone)]
pub struct Qemu {
    pub cpu: Option<Rv64>,
    pub serial: Option<Uart>,
    pub timer: Option<ClintTimer>,
    pub intc: Option<Plic>,
}

impl Qemu {
    pub fn new() -> Self {
        let cpu = Rv64::new(0);
        let uart = Uart::new(0x1000_0000);
        let ctimer = ClintTimer::new(0x0200_4000);
        let intc = Plic::new(0x0c00_0000);
        Qemu {
            cpu: Some(cpu),
            serial: Some(uart),
            timer: Some(ctimer),
            intc: Some(intc),
        }
    }
}

impl HasSerial for Qemu {
    type Device = Uart;

    /* シリアルデバイスの取得 */
    fn take_serial(&mut self) -> <Self as HasSerial>::Device {
        let p = replace(&mut self.serial, None);
        p.unwrap()
    }

    /* シリアルデバイスの解放 */
    fn release_serial(&mut self, serial: <Self as HasSerial>::Device) {
        let p = replace(&mut self.serial, Some(serial));
    }
}

impl HasTimer for Qemu {
    type Device = ClintTimer;

    /* タイマの取得 */
    fn take_timer(&mut self) -> <Self as HasTimer>::Device {
        let p = replace(&mut self.timer, None);
        p.unwrap()
    }

    /* タイマの解放 */
    fn release_timer(&mut self, timer: <Self as HasTimer>::Device) {
        let p = replace(&mut self.timer, Some(timer));
    }
}

impl HasCpu for Qemu {
    type Device = Rv64;

    /* CPUの取得 */
    fn take_cpu(&mut self) -> <Self as HasCpu>::Device {
        let p = replace(&mut self.cpu, None);
        p.unwrap()
    }

    /* CPUの解放 */
    fn release_cpu(&mut self, cpu: <Self as HasCpu>::Device) {
        let p = replace(&mut self.cpu, Some(cpu));
    }
}

impl HasIntc for Qemu {
    type Device = Plic;

    /* 割込みコントローラの取得 */
    fn take_intc(&mut self) -> <Self as HasIntc>::Device {
        let p = replace(&mut self.intc, None);
        p.unwrap()
    }

    /* 割込みコントローラの解放 */
    fn release_intc(&mut self, intc: <Self as HasIntc>::Device) {
        let p = replace(&mut self.intc, Some(intc));
    }
}
