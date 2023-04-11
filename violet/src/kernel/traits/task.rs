//! タスクトレイト

pub trait TraitTask {
    fn new(id: u64, func: fn()) -> Self;
    fn get_entry(&self) -> fn();
    fn set_priority(&mut self, prio: u64);
    fn get_priority(&self) -> u64;
}