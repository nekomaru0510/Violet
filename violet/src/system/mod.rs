//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

use core::intrinsics::transmute;

use crate::system::config::{get_container_id, get_container_bsp};
use crate::kernel::heap::init_allocater;
use crate::kernel::init_calls::do_app_calls;
use crate::container::{get_container, get_mut_container, is_ready_container};

#[cfg(test)]
use crate::test_entry;

pub mod config;

extern "C" {
    static __HEAP_BASE: usize;
    static __HEAP_END: usize;
}

pub fn init_system(cpu_id: usize) {
    
    // Initialize memory allocator
    unsafe {
        init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));
    }

    enter_container(cpu_id);
}

pub fn enter_container(cpu_id: usize) {
    
    let container_id = get_container_id(cpu_id);

    // check this core is container's bsp
    if get_container_bsp(container_id) == cpu_id {
        // Setup container
        do_app_calls();
        // Run self container
        get_mut_container().run();
    } else {
        // Wait until the container is ready
        while !is_ready_container(container_id) {};
        // Entry to the container
        get_container().entry();
    }
}
