//! Running Linux on virtual machine with violet
#![no_main]
#![no_std]
#![feature(used_with_arg)]

extern crate violet;
use violet::kernel::syscall::vsi::create_task;

mod linux;
mod freertos;
use crate::linux::boot_linux;
use crate::freertos::boot_freertos;

use violet::app_init;
app_init!(main);

pub fn main() {
    // Boot Linux on core 1
    create_task(2, boot_linux, 1);
    // Boot FreeRTOS on core 0
    create_task(3, boot_freertos, 0);
}
