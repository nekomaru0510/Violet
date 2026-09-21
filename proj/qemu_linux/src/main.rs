//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

#![no_main]
#![no_std]
#![feature(used_with_arg)]

pub mod setup;
pub mod vm_linux;

extern crate violet;
use violet::kernel::syscall::vsi::create_task;

use crate::vm_linux::boot_linux;

pub fn main() {
    // Boot Linux on core 1
    create_task(2, boot_linux, 1);
}