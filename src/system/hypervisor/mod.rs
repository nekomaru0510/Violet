//! Hypervisor機能本体

use crate::PERIPHERALS;
use crate::driver::arch::rv64::*;
use core::fmt::{self, Write};
use crate::driver::traits::cpu::TraitCpu;
//use crate::system::hypervisor::print;
//use crate::driver::traits::serial::TraitSerial;

use crate::environment::traits::cpu::HasCpu;

use crate::print;
use crate::println;
//use crate::library::std::println;

pub fn boot_guest() {
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    cpu.enable_interrupt();
    cpu.set_default_vector();
    jump_guest_kernel(0x8020_0000, 0, 0x8220_0000);    
}

/* 一応、何らかの設定値を格納できるように */
pub struct Hypervisor {
    sched: i32,
}

impl Hypervisor {
    pub fn new() -> Hypervisor {
        Hypervisor{sched: 0, }
    }

    pub fn run(&self) {
        println!("Hello I'm {} ", "Violet Hypervisor");
        boot_guest();
    }
}
