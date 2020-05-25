//! Interrupt Controller module

use crate::kernel::driver::board::sifive_u::plic::Plic;

pub struct Irq {
    irq: Plic,
}

pub trait IrqAttr {
    fn new() -> Self;
    fn enable_interrupt(&self, id: u64);
    //fn disable_interrupt(&self, id: u64);
}

/*
/// Violet内共通割込み番号
/// 実際のH/Wの割込み番号とは異なる
enum InterruptId {
    TIMER = 1,
}
*/

impl Irq {
    pub fn new() -> Self {
        Irq {irq: Plic::new(),}
    }

    pub fn enable(&self, id:u64) {
        self.irq.enable_interrupt(id);
    }

}

