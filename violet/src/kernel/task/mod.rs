//!タスク
use crate::kernel::traits::task::TraitTask;

pub struct Task {
    id: u64,
    func: fn(),
    prio: u64,
}

impl TraitTask for Task {
    fn new(id: u64, func: fn()) -> Self {
        Task { id, func, prio: 32 }
    }

    fn get_entry(&self) -> fn() {
        self.func
    }

    fn set_priority(&mut self, prio: u64) {
        self.prio = prio;
    }

    fn get_priority(&self) -> u64 {
        self.prio
    }
}
