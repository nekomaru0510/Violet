//! CPU用のトレイト

pub mod mmu;

pub trait TraitCpu {
    /* 割込みの有効化 */
    fn enable_interrupt(&self);
    /* 割込みの無効化 */
    fn disable_interrupt(&self);
}
