//! Interrupt Controller module
#![no_std]

use plic::Plic;
use class::irq::IrqAttr;

pub struct Irq {
    irq: Plic,
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

