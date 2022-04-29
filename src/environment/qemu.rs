//! QEMU(RISCV virt)

use core::mem::replace;

use crate::PERIPHERALS;

/* デバイスドライバ */
use crate::driver::arch::rv64::*;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::uart::Uart;
use crate::driver::board::sifive_u::plic::Plic;

/* ドライバ用トレイト */
//use crate::driver::traits::serial::TraitSerial;
//use crate::driver::traits::timer::TraitTimer;

use crate::environment::traits::serial::HasSerial;
use crate::environment::traits::timer::HasTimer;
use crate::environment::traits::cpu::HasCpu;

pub fn init_peripherals() {
    unsafe {
        PERIPHERALS.cpu = Some(Processor::new(0));
        PERIPHERALS.serial = Some(Uart::new(0x1000_0000));
        PERIPHERALS.timer = Some(ClintTimer::new(0x0200_4000));
    }
}

#[derive(Clone)]
pub struct Qemu {    
    pub cpu: Option<Processor>,
    pub serial: Option<Uart>,
    pub timer: Option<ClintTimer>, 
}

impl Qemu
{
    pub fn new() -> Self {
        let cpu = Processor::new(0);
        let uart = Uart::new(0x1000_0000);
        let ctimer = ClintTimer::new(0x0200_4000);
        Qemu { cpu: Some(cpu), serial:Some(uart), timer:Some(ctimer), }
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
    fn release_serial(&mut self, serial: <Self as HasSerial>::Device ) {
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
    fn release_timer(&mut self, timer: <Self as HasTimer>::Device ) {
        let p = replace(&mut self.timer, Some(timer));
    }
}

impl HasCpu for Qemu {
    type Device = Processor;
    
    /* CPUの取得 */
    fn take_cpu(&mut self) -> <Self as HasCpu>::Device {
        let p = replace(&mut self.cpu, None);
        p.unwrap()
    }
    
    /* CPUの解放 */
    fn release_cpu(&mut self, cpu: <Self as HasCpu>::Device ) {
        let p = replace(&mut self.cpu, Some(cpu));
    }
}