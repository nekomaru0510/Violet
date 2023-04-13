#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
/* テスト用 */
//#![feature(custom_test_frameworks)]
//#![test_runner(crate::container::hypervisor_container::test_runner)]
//#![reexport_test_harness_main = "test_main"]
/* warning抑制 */
#![allow(dead_code)]
#![allow(unused_variables)]
#![no_std]

/* Violetの中核機能(なるべく小さくしたい) */
pub mod kernel;

pub mod driver;
pub mod environment;
pub mod library;
pub mod system;

/* 環境依存 */
use crate::driver::arch::rv64::Rv64;
use crate::environment::qemu::init_peripherals;
use crate::environment::qemu::wakeup_all_cpus;
use crate::environment::qemu::Qemu;

pub static CPU: Rv64 = Rv64::new(0);
pub static mut PERIPHERALS: Qemu = Qemu {
    cpu: None,
    serial: None,
    timer: None,
    intc: None,
};

/* システム依存 */
use crate::system::vm::VirtualMachine;

use crate::kernel::init_calls::do_init_calls;
use crate::kernel::heap::init_allocater;
use crate::kernel::main_loop;
use crate::kernel::syscall::toppers::{T_CTSK, cre_tsk};

extern crate core;
use core::intrinsics::transmute;

extern "C" {
    static __HEAP_BASE: usize;
    static __HEAP_END: usize;
}

#[no_mangle]
pub extern "C" fn boot_init(cpu_id: usize) {    
//pub extern "C" fn boot_init(cpu_id: usize) -> ! {    
    /* メモリアロケータの初期化 */
    unsafe {init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));}
    //init_allocater(0x8004_0000, 0x8006_0000);
    #[cfg(test)]
    test_main();

    init_peripherals();

    println!("Hello I'm {} ", "Violet Hypervisor");

    // CPU0にinit_callsを実行させる
    cre_tsk(1, &T_CTSK{task:do_init_calls, prcid:0});
    // 他CPUをすべて起動させる
    wakeup_all_cpus(cpu_id);

    main_loop(cpu_id);
}

/* 無いとコンパイルエラー(言語仕様) */
use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* 無いとコンパイルエラー */
#[no_mangle]
pub extern "C" fn abort(_info: &PanicInfo) -> ! {
    loop {}
}
