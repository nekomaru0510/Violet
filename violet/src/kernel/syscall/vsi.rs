//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Violet Systemcall Interface (VSI)

use crate::kernel::task::Task;
use crate::kernel::traits::sched::TraitSched;
use crate::kernel::traits::task::TraitTask;
use crate::kernel::SCHEDULER;

pub fn create_task(taskid: u64, task: fn(), prcid: usize) {
    let task = Task::new(taskid, task);
    unsafe {
        SCHEDULER[prcid].register(task);
    }
}
