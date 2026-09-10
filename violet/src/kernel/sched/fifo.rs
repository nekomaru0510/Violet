//! FIFO scheduler
extern crate alloc;
use alloc::vec::Vec;
use spin::Mutex;

use crate::kernel::task::Task;

use crate::kernel::traits::sched::TraitSched;
use crate::kernel::traits::task::TraitTask;

pub struct FifoScheduler<T: TraitTask> {
    task_queue: Mutex<Vec<Option<T>>>,
}

impl FifoScheduler<Task> {
    pub const fn new() -> Self {
        FifoScheduler {
            task_queue: Mutex::new(Vec::new()),
        }
    }
}

impl<T: TraitTask> TraitSched<T> for FifoScheduler<T> {
    fn next(&mut self) -> Option<T> {
        match self.task_queue.lock().pop() {
            None => None,
            Some(task) => task,
        }
    }

    fn register(&mut self, task: T) {
        self.task_queue.lock().insert(0, Some(task));
    }

    fn unregister(&mut self) {
        self.task_queue.lock().remove(0);
    }
}
