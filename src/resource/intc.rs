//! Interrupt Controller module

/* リソース用トレイト */
//use crate::resource::traits::tty::TraitTty;

/* ドライバ用トレイト */
use crate::driver::traits::intc::TraitIntc;

pub struct Intc<T: TraitIntc> {
    intc: T,
}

/*
/// Violet内共通割込み番号
/// 実際のH/Wの割込み番号とは異なる
enum InterruptId {
    TIMER = 1,
}
*/

impl<T> Intc<T>
where
    T: TraitIntc,
{
    pub fn new(intc: T) -> Self {
        Intc { intc }
    }

    
}

/*
impl<T> Trait*** for Intc<T>
where
    T: TraitIntc,
{
    pub fn new() -> Self {
        Irq {irq: Plic::new(),}
    }

    pub fn enable(&self, id:u64) {
        self.irq.enable_interrupt(id);
    }

}
*/
