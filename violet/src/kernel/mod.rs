pub mod init_calls;
pub mod heap;
pub mod dispatcher;
pub mod sched;
pub mod task;
pub mod traits;
pub mod syscall;

use crate::kernel::traits::dispatcher::TraitDispatcher;
use crate::kernel::traits::task::TraitTask;
use crate::kernel::traits::sched::TraitSched;

use crate::CPU;

use sched::fifo::FifoScheduler;
use dispatcher::minimal_dispatcher::MinimalDispatcher;
use task::Task;

pub fn idle_core() {
    CPU.inst.wfi();
}

/* [todo fix] 本来はコアごと？にスケジューラ、ディスパッチャを指定したい */
pub static mut SCHEDULER: [FifoScheduler<Task>; 2] = [
    FifoScheduler::new(),
    FifoScheduler::new(),
];

pub static mut DISPATCHER: [MinimalDispatcher; 2] = [
    MinimalDispatcher::new(),
    MinimalDispatcher::new(),
];

pub fn main_loop(cpu_id: usize) {
    loop {
        unsafe {
            let task = SCHEDULER[cpu_id].next();
            match task {
                None => idle_core(),
                Some(t) => {
                    DISPATCHER[cpu_id].dispatch(t);
                },
            }
        }
    }
}