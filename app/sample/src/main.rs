#![no_main]
#![no_std]

extern crate kernel;

use kernel::{print, println};
//use kernel::system::hypervisor::INIT_CALLS;
use kernel::driver::traits::arch::riscv::Registers;

#[link_section = ".init_calls"]
#[no_mangle]
pub static mut INIT_CALLS: Option<fn(&mut Registers)> = Some(init_sample);

pub fn init_sample(regs: &mut Registers) {
    println!("sample init !!");
}

