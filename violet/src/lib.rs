//! Violet Hypervisor
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)] /* [todo remove] */
/* テスト用 */
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
/* warning抑制 */
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(static_mut_refs)] /* [todo remove] */
#![no_std]

extern crate core;
extern crate alloc;

pub mod arch;
pub mod container;
pub mod driver;
pub mod environment;
pub mod kernel;
pub mod library;
pub mod resource;
pub mod utils;
#[cfg(test)]
pub mod test;

/*
 * [todo fix] I want to place "test_entry" in the test module, 
 * but I can't refer to test_main, so I'll place it here.)
 */
#[no_mangle]
pub fn test_entry() {
    #[cfg(test)]
    test_main();
}
