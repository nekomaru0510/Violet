//! Kernel

pub mod container;
pub mod dispatcher;
pub mod heap;
pub mod init_calls;
pub mod sched;
pub mod syscall;
pub mod task;
pub mod traits;

use crate::CPU;

use crate::environment::init_environment;
use crate::environment::NUM_OF_CPUS;
use crate::print;
use crate::println;
#[cfg(test)]
use crate::test_entry;

use container::*;
use dispatcher::minimal_dispatcher::MinimalDispatcher;
use heap::init_allocater;
use init_calls::*;
use sched::fifo::FifoScheduler;
use syscall::vsi::create_task;
use task::Task;

use traits::dispatcher::TraitDispatcher;
use traits::sched::TraitSched;

use crate::driver::arch::rv64::boot::_start_ap; // [todo delete]
use crate::driver::arch::rv64::sbi; // [todo delete]

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

    /* ルートコンテナの生成 */
    create_container();

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
    let con = get_mut_container(0); // RootContainerの取得
    match &con.unwrap().cpu[cpu_id] {
        None => (),
        Some(c) => c.core_init(),
    }
}

fn init_ap(cpu_id: usize) {
    let con = get_mut_container(0); // RootContainerの取得
    match &con.unwrap().cpu[cpu_id] {
        None => (),
        Some(c) => c.core_init(),
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
    CPU.inst.wfi();
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
