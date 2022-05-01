//! Hypervisor機能本体

extern crate alloc;
use alloc::string::String;

use crate::PERIPHERALS;

use crate::environment::traits::cpu::HasCpu;
use crate::driver::traits::cpu::TraitCpu;

use crate::driver::arch::rv64::*; /* todo delete*/

use crate::driver::traits::arch::riscv::TraitRisvCpu;
use crate::driver::traits::arch::riscv::Registers;

use crate::library::vshell::{VShell, Command};

use crate::print;
use crate::println;

fn echo_test(exc_num: usize, regs: Registers) {
    println!("exceptioin occur!: {}", exc_num);
}

pub fn boot_guest() {
    
    switch_hs_mode(0x8020_0000, 0, 0x8220_0000);

    let cpu = unsafe { PERIPHERALS.take_cpu() };
    
    cpu.enable_interrupt();
    cpu.set_default_vector();
    cpu.register_exception(10, echo_test);
    cpu.register_exception(7, echo_test);
    unsafe { PERIPHERALS.release_cpu(cpu) };
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

        
        let mut vshell = VShell::new();
        vshell.add_cmd(Command{name: String::from("boot"), func: boot_guest});
        vshell.run();
        //boot_guest();
    }
}
