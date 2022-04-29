//! QEMU(RISCV virt)

use core::mem::replace;

/* デバイスドライバ */
use crate::driver::arch::rv64::*;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::uart::Uart;
use crate::driver::board::sifive_u::plic::Plic;

/* ドライバ用トレイト */
use crate::driver::traits::serial::TraitSerial;
use crate::driver::traits::timer::TraitTimer;


static mut PERIPHERALS: Qemu = Qemu {
    serial: None,
    timer: None,
};

#[derive(Clone)]
pub struct Qemu {    
    serial: Option<Uart>,
    timer: Option<ClintTimer>, 
}

impl Qemu
{
    pub fn new() -> Self {
        let uart = Uart::new(0x1000_0000);
        let ctimer = ClintTimer::new(0x0200_4000);
        Qemu { serial:Some(uart), timer:Some(ctimer), }
    }

    /* シリアルポートの取得 */
    /* 呼出し元は、 ジェネリック型として受け取る。トレイト境界は設定する */
    fn take_serial(&mut self) -> Uart {
        let p = replace(&mut self.serial, None);
        p.unwrap()
    }
}


/* トレイトを使いたいが、グローバル変数がうまく定義できないので、ペンディング */
/* Noneには、トレイトが適用できないので、そもそも無理？ */
/*



//static mut PERIPHERALS: Qemu<T: TraitSerial = Uart, U: TraitTimer = ClintTimer> = Qemu {
//static mut PERIPHERALS: Qemu<T: TraitSerial, U: TraitTimer> = Qemu {    
//static mut PERIPHERALS: Qemu<Option<Uart>, Option<ClintTimer>, T: TraitSerial, U: TraitTimer> = Qemu {
//static mut PERIPHERALS: Qemu<Option<T=Uart>, Option<U=ClintTimer>> = Qemu {
static mut PERIPHERALS: Qemu<Option<Uart>, Option<ClintTimer>> = Qemu {
//static mut PERIPHERALS: Qemu<TraitSerial, TraitTimer> = Qemu {    
///static mut PERIPHERALS: Qemu<T:Uart, U:ClintTimer> = Qemu {    
///static mut PERIPHERALS = Qemu<T: TraitSerial = Uart, U: TraitTimer = ClintTimer> {
    serial: None,
    timer: None,
};

#[derive(Clone)]
//pub struct Qemu<T: TraitSerial, U: TraitTimer> {
pub struct Qemu<T: TraitSerial, U: TraitTimer> {    
    serial: Option<T>,
    timer: Option<U>, 
}

///impl<Option<T>, Option<U>> Qemu<T, U>
///impl<Option<T>, Option<U>> Qemu<Option<T>, Option<U>>
//impl<T, U> Qemu<Option<T>, Option<U>>
impl<T, U> Qemu<T, U>
where
    T: TraitSerial,
    U: TraitTimer,
{
    pub fn new() -> Self {
        let uart = Uart::new(0x1000_0000);
        let ctimer = ClintTimer::new(0x0200_4000);
        Qemu { serial:Some(uart), timer:Some(ctimer), }
    }

    /* シリアルポートの取得 */
    fn take_serial(&mut self) -> T {
        let p = replace(&mut self.serial, None);
        p.unwrap()
    }
}

*/