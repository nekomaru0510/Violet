//! Violet Hypervisor
#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
/* テスト用 */
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
/* warning抑制 */
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![no_std]

pub mod driver;
pub mod environment;
pub mod kernel;
pub mod library;
#[cfg(test)]
pub mod test;

/*
 * [todo fix] 本来は、testモジュール内に配置したいが、
 * test_mainを参照できないため、ここに配置
 */
#[no_mangle]
pub fn test_entry() {
    #[cfg(test)]
    test_main();
}
