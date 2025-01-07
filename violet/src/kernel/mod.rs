//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Kernel

pub mod dispatcher;
pub mod heap;
pub mod init_calls;
mod panic;
pub mod sched;
pub mod syscall;
pub mod task;
pub mod traits;

use crate::container::{get_container, get_mut_container, does_container_exist};
//use crate::{print, println};

use dispatcher::minimal_dispatcher::MinimalDispatcher;
use sched::fifo::FifoScheduler;
use syscall::vsi::create_task;
use task::Task;
use crate::environment::Arch;
use crate::arch::traits::TraitArch;

use traits::dispatcher::TraitDispatcher;
use traits::sched::TraitSched;

use crate::arch::rv64::instruction::Instruction; // [todo delete]


fn idle_core() {
    Instruction::wfi();
}

extern crate alloc;
use alloc::boxed::Box;
use heap::{TraitHeap, HEAP};

pub struct Kernel {
    heap: Box<&'static mut (dyn TraitHeap + 'static)>,
}

impl Kernel {
    pub fn new(heap: Box<&'static mut (dyn TraitHeap + 'static)>) -> Self {
        Kernel { heap }
    }

    pub fn create_custom_kernel(container_id: usize) -> Self {
        Kernel::new(Box::new(unsafe { &mut HEAP }))
    }

    pub fn run(&self) {
        main_loop(Arch::get_cpuid());
    }

    pub fn entry(&self) {
        main_loop(Arch::get_cpuid());
    }
}

pub fn get_kernel() -> &'static Kernel {
    &get_container().kernel
}

pub fn get_mut_kernel() -> &'static mut Kernel {
    &mut get_mut_container().kernel
}

// [todo fix] Select scheduler and dispatcher for each core
pub static mut SCHEDULER: [FifoScheduler<Task>; 2] = [FifoScheduler::new(), FifoScheduler::new()];

pub static mut DISPATCHER: [MinimalDispatcher; 2] =
    [MinimalDispatcher::new(), MinimalDispatcher::new()];

pub fn main_loop(cpu_id: usize) {
    loop {
        unsafe {
            let task = SCHEDULER[cpu_id].next();
            match task {
                None => (), //idle_core(),
                Some(t) => {
                    DISPATCHER[cpu_id].dispatch(&t);
                }
            }
        }
    }
}
