//! Kernel

pub mod dispatcher;
pub mod heap;
pub mod init_calls;
mod panic;
pub mod sched;
pub mod syscall;
pub mod task;
pub mod traits;

use crate::container::{get_container, get_mut_container};
use crate::environment::init_environment;
use crate::environment::NUM_OF_CPUS;
use crate::print;
use crate::println;
use crate::resource::{get_resources, BorrowResource, ResourceType};
#[cfg(test)]
use crate::test_entry;

use dispatcher::minimal_dispatcher::MinimalDispatcher;
use heap::init_allocater;
use init_calls::*;
use sched::fifo::FifoScheduler;
use syscall::vsi::create_task;
use task::Task;

use traits::dispatcher::TraitDispatcher;
use traits::sched::TraitSched;

use crate::arch::rv64::boot::_start_ap; // [todo delete]
use crate::arch::rv64::instruction::Instruction; // [todo delete]
use crate::arch::rv64::sbi; // [todo delete]
                                    //use crate::driver::traits::cpu::TraitCpu;

extern crate core;
use core::intrinsics::transmute;

extern "C" {
    static __HEAP_BASE: usize;
    static __HEAP_END: usize;
}

#[no_mangle]
pub extern "C" fn boot_init(cpu_id: usize) {
    /* メモリアロケータの初期化 */
    unsafe {
        init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));
    }

    init_environment();
    do_driver_calls();

    init_bsp(cpu_id);

    println!("Hello I'm {} ", "Violet Hypervisor");

    #[cfg(test)]
    test_entry();

    // CPU0にinit_callsを実行させる
    create_task(1, do_app_calls, 0);

    // 他CPUをすべて起動させる
    wakeup_all_cpus(cpu_id);

    main_loop(cpu_id);
}

fn init_bsp(cpu_id: usize) {
    if let BorrowResource::Cpu(c) = get_resources().get(ResourceType::Cpu, cpu_id) {
        c.core_init()
    }
}

fn init_ap(cpu_id: usize) {
    if let BorrowResource::Cpu(c) = get_resources().get(ResourceType::Cpu, cpu_id) {
        c.core_init()
    }

    main_loop(cpu_id);
}

fn wakeup_all_cpus(cpu_id: usize) {
    for i in 0..NUM_OF_CPUS {
        if i as usize != cpu_id {
            sbi::sbi_hart_start(i as u64, _start_ap as u64, init_ap as u64); /* [todo fix] CPU起床は抽象かする */
        }
    }
}

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
}

pub fn get_kernel() -> &'static Kernel {
    &get_container().kernel
}

pub fn get_mut_kernel() -> &'static mut Kernel {
    &mut get_mut_container().kernel
}

/* [todo fix] 本来はコアごと？にスケジューラ、ディスパッチャを指定したい */
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
