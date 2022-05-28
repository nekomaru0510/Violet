#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
/* テスト用 */
#![feature(custom_test_frameworks)]
#![test_runner(crate::container::hypervisor_container::test_runner)]
#![reexport_test_harness_main = "test_main"]
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
use crate::environment::qemu::init_peripherals;
use crate::environment::qemu::Qemu;

pub static mut PERIPHERALS: Qemu = Qemu {
    cpu: None,
    serial: None,
    timer: None,
    intc: None,
};

/* システム依存 */
use crate::system::hypervisor::Hypervisor;

use crate::kernel::init_calls::do_init_calls;
use crate::kernel::slab_allocator::init_allocater;

#[no_mangle]
pub extern "C" fn boot_init() -> ! {
    /* メモリアロケータの初期化 */
    init_allocater(0x8004_0000, 0x8006_0000);

    #[cfg(test)]
    test_main();

    init_peripherals();

    do_init_calls();

    /* システムの起動 */
    let hv = Hypervisor::new();
    hv.run();

    loop {}
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
