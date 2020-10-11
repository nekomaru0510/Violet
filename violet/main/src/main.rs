#![no_main]

#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]

#![no_std]

use kernel::Kernel;

#[allow(unused_imports)]
use entry::*;
use std::*;

use vshell::VShell;
use vshell::KernelThread;

// test
//use crate::kernel::resource::io::fesyscall::FeSyscall;

#[no_mangle]
pub extern "C" fn boot_init() -> ! {

    let mut kernel = Kernel::new();
    kernel.run();

    //println!("Hello I'm {}!! ver.{}", "Violet", 0.1);
    println!("Hello I'm {}!! ", "Violet");


    let mut vs = VShell::new();
    vs.run();

    println!("Good Bye!!");

    loop{}
}

/* 無いとコンパイルエラー(言語仕様) */
use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

/* 無いとコンパイルエラー */
#[no_mangle]
pub extern "C" fn abort(_info: &PanicInfo) -> ! {
    loop{}
}
