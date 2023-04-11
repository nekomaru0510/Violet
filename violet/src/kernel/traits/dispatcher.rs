//! ディスパッチャ用のトレイト
use crate::kernel::traits::task::TraitTask;

pub trait TraitDispatcher <T:TraitTask> {
    fn dispatch(&self, task :&T);
}
