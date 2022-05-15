//! タイマ用のトレイト

pub trait TraitTimer {
    //fn new() -> Self;
    fn write(&self, t: u64);
    fn read(&self) -> u64;
    fn enable_interrupt(&self);
    fn disable_interrupt(&self);
    fn set_interrupt_time(&self, t: u64); // いるかどうか要検討
}