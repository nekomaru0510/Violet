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

pub mod kernel;     /* Violetの中核機能(なるべく小さくしたい) */
pub mod driver;
pub mod environment;
pub mod library;
pub mod system;

/* [todo delete]環境依存 */
use crate::driver::arch::rv64::Rv64;
pub static CPU: Rv64 = Rv64::new(0);

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
