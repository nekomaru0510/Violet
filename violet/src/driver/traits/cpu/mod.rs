//! CPU用のトレイト

pub mod context;
pub mod mmu;
pub mod registers;

pub trait TraitCpu {
    /* コアごとの初期化 */
    fn core_init(&self);

    /* CPUのstart */
    fn wakeup(&self);
    /* CPUの停止 */
    fn sleep(&self);
    /* 割込みの有効化 */
    fn enable_interrupt(&self);
    /* 割込みの無効化 */
    fn disable_interrupt(&self);
}
