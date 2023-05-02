//! スケジューラ用のトレイト
use crate::kernel::traits::task::TraitTask;

pub trait TraitSched<T: TraitTask> {
    fn next(&mut self) -> Option<T>;
    fn register(&mut self, task: T);
    fn unregister(&mut self);
}
