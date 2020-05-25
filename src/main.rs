#![no_main]

#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![feature(associated_type_bounds)]

#![no_std]

#[macro_use]
mod kernel;
mod app;

use kernel::Kernel;

use crate::kernel::interface::vkth::lib::*;

use crate::app::KernelThread;
use crate::app::vshell::VShell;

#[no_mangle]
pub extern "C" fn boot_init() -> ! {
    
    let mut kernel = Kernel::new();
    kernel.run();

    println!("Hello I'm {}!! ver.{}", "Violet", 0.01);
    
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
