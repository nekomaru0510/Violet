//! Kernel 

pub mod init_calls;
pub mod heap;
pub mod dispatcher;
pub mod sched;
pub mod task;
pub mod traits;
pub mod syscall;
pub mod container;

use crate::CPU;

use crate::environment::qemu::init_peripherals;
use crate::println;
use crate::print;

use init_calls::*;
use heap::init_allocater;
use syscall::toppers::{T_CTSK, cre_tsk};
use container::*;
use sched::fifo::FifoScheduler;
use dispatcher::minimal_dispatcher::MinimalDispatcher;
use task::Task;

use traits::dispatcher::TraitDispatcher;
use traits::task::TraitTask;
use traits::sched::TraitSched;

use crate::driver::arch::rv64::boot::_start_ap;// [todo delete]
use crate::driver::arch::rv64::sbi; // [todo delete]

static NUM_OF_CPUS: usize = 2;

extern crate core;
use core::intrinsics::transmute;

extern "C" {
    static __HEAP_BASE: usize;
    static __HEAP_END: usize;
}

#[no_mangle]
pub extern "C" fn boot_init(cpu_id: usize) {    
    /* メモリアロケータの初期化 */
    unsafe {init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));}

    #[cfg(test)]
    test_main();

    create_container();

    init_peripherals();
    do_driver_calls();

    println!("Hello I'm {} ", "Violet Hypervisor");

    // CPU0にinit_callsを実行させる
    cre_tsk(1, &T_CTSK{task:do_app_calls, prcid:0});
    // 他CPUをすべて起動させる
    wakeup_all_cpus(cpu_id);

    main_loop(cpu_id);
}

pub fn wakeup_all_cpus(cpu_id: usize) {
    for i in 1 .. NUM_OF_CPUS+1 {
        if i as usize != cpu_id {
            sbi::sbi_hart_start(i as u64 , _start_ap as u64, main_loop as u64); /* [todo fix] CPU起床は抽象かする */
        }
    }
}

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