//! 最小構成ディスパッチャ

use crate::kernel::traits::dispatcher::TraitDispatcher;
use crate::kernel::traits::task::TraitTask;

pub struct MinimalDispatcher {

}

impl MinimalDispatcher {
    pub const fn new() -> Self {
        MinimalDispatcher{}
    }
}

impl<T:TraitTask> TraitDispatcher<T> for MinimalDispatcher {
    fn dispatch(&self, task :&T) {
        ((*task).get_entry())()
    }
}