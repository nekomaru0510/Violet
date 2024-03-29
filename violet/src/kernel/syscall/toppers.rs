//! Toppersシステムコールインタフェース

use crate::kernel::task::Task;
use crate::kernel::traits::sched::TraitSched;
use crate::kernel::traits::task::TraitTask;
use crate::kernel::SCHEDULER;

pub struct Ctsk {
    /*
    tskatr: ATR,
    exinf: EXINF,
    */
    pub task: fn(),
    /*
    itskpri: PRI,
    stksz: size_t,
    stk: &STK_T,
    sstksz: size_t,
    sstk: &STK_T,
    */
    pub prcid: usize,
}

pub fn cre_tsk(tskid: usize, ctsk: &Ctsk) {
    let task = Task::new(tskid as u64, ctsk.task);
    unsafe {
        SCHEDULER[ctsk.prcid].register(task);
    }
}
