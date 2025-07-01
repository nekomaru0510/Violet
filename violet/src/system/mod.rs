// SPDX-License-Identifier: MIT 
// SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! System initialization and container entry logic for the Violet hypervisor.
//! 
//! - Initializes memory allocator and system state
//! - Handles entry into the appropriate container for each CPU

use core::intrinsics::transmute;

use crate::system::config::{get_container_id, get_container_bsp};
use crate::kernel::heap::init_allocater;
use crate::kernel::init_calls::do_app_calls;
use crate::container::{get_container, get_mut_container, is_ready_container};

pub mod config;

extern "C" {
    /// Start address of the heap (provided by linker script)
    static __HEAP_BASE: usize;
    /// End address of the heap (provided by linker script)
    static __HEAP_END: usize;
}

/// System-wide initialization finished flag
static mut SYSTEM_INIT_FINISHED: bool = false;

/// Initialize the system and enter the container for the given CPU.
///
/// # Arguments
/// * `cpu_id` - Physical CPU ID to initialize
pub fn init_system(cpu_id: usize) {
    // Initialize memory allocator
    unsafe {
        init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));
    }
    // Mark system initialization as finished
    unsafe {
        SYSTEM_INIT_FINISHED = true;
    }
    // Enter the container for this CPU
    enter_container(cpu_id);
}

/// Enter the container for the given CPU.
///
/// # Arguments
/// * `cpu_id` - Physical CPU ID
pub fn enter_container(cpu_id: usize) {
    // Wait until system initialization is finished (busy-wait)
    while !unsafe { SYSTEM_INIT_FINISHED } {
        // Wait until system initialization is finished
    }
    let container_id = get_container_id(cpu_id);
    // Check if this core is the container's BSP (bootstrap processor)
    if get_container_bsp(container_id) == cpu_id {
        // Setup container
        do_app_calls();
        // Run self container
        get_mut_container().run();
    } else {
        // Wait until the container is ready
        while !is_ready_container(container_id) {}
        // Entry to the container
        get_container().entry();
    }
}
