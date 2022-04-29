//! Hypervisor機能本体

use crate::environment::qemu::PERIPHERALS;
use crate::driver::arch::rv64::*;
use core::fmt::{self, Write};
use crate::driver::traits::cpu::TraitCpu;
//use crate::system::hypervisor::print;

#[macro_export]
macro_rules! print2{
     ($($arg:tt)*) => (print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println2 {
    ($fmt:expr) => (print2!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print2!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    let mut serial = unsafe { PERIPHERALS.take_serial() };
    serial.write_fmt(args).unwrap();
    unsafe { PERIPHERALS.return_serial(serial) };
}

pub fn boot_guest() {
    println2!("Hello I'm {} ", "Violet Hypervisor");
    let cpu = unsafe { PERIPHERALS.take_cpu() };
    cpu.enable_interrupt();
    cpu.set_default_vector();
    jump_guest_kernel(0x8020_0000, 0, 0x8220_0000);    
}

