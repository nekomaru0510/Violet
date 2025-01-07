//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

use crate::kernel::heap::init_allocater;
use core::intrinsics::transmute;
use crate::container::{get_container, get_mut_container, get_container_by_id, does_container_exist, is_ready_container};
use crate::system::config::{get_container_id, get_container_bsp};
use crate::environment::NUM_OF_CPUS;
use crate::arch::rv64::boot::_start_ap; // [todo delete]
use crate::resource::{get_resources, BorrowResource, ResourceType};
use crate::arch::rv64::sbi; // [todo delete]
use crate::kernel::init_calls::do_app_calls;

#[cfg(test)]
use crate::test_entry;

pub mod config;

extern "C" {
    static __HEAP_BASE: usize;
    static __HEAP_END: usize;
}

#[no_mangle]
pub extern "C" fn boot_init(cpu_id: usize) {
    
    // System initialization
    // Initialize memory allocator
    unsafe {
        init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));
    }
    // Wake up all CPUs
    wakeup_all_cpus(cpu_id);

    //println!("Hello I'm {} ", "Violet Hypervisor");

    #[cfg(test)]
    test_entry();

    // Setup containers
    // Run init_calls on CPU0
    if does_container_exist() {
        get_container().entry();
    } else {
        do_app_calls();
    }
    
    // Run self container
    get_mut_container().run();
}


pub fn system_init(cpu_id: usize) {
    
    // System initialization
    // Initialize memory allocator
    unsafe {
        init_allocater(transmute(&__HEAP_BASE), transmute(&__HEAP_END));
    }
    // Wake up all CPUs
    wakeup_all_cpus(cpu_id);

    enter_container(cpu_id);
}

use crate::environment::Arch;
use crate::arch::traits::TraitCpu;

pub fn enter_container(cpu_id: usize) {
    
    let mut container_id = 0;

    // Get Container ID from System Configuration
    while (container_id) == 0 {
        container_id = get_container_id(cpu_id);
    };
    // Set Container ID
    Arch::get_mut_core().set_container_id(container_id);

    // check this core is container's bsp
    if get_container_bsp(container_id) == cpu_id {
        // Setup container
        do_app_calls();
        // Run self container
        get_mut_container().run();
    } else {
        while !is_ready_container(container_id) {};
        // Entry to the container
        get_container().entry();
    }
}



fn init_ap(cpu_id: usize) {
    
    let mut container_id = 0;

    // Get Container ID from System Configuration
    while (container_id) == 0 {
        container_id = get_container_id(cpu_id);
    };
    // Set Container ID
    Arch::get_mut_core().set_container_id(container_id);

    // Wait until container is ready
    while !is_ready_container(container_id) {};

    // Wait until container is running
    while !get_container_by_id(container_id).is_running() {};

    // check this core is container's bsp
    if get_container_bsp(container_id) == cpu_id {
        // Run self container
        //get_mut_container().run();
        get_container().entry();
    } else {
        // Entry to the container
        get_container().entry();
        
    }
}

fn wakeup_all_cpus(cpu_id: usize) {
    for i in 0..NUM_OF_CPUS {
        if i as usize != cpu_id {
            //sbi::sbi_hart_start(i as u64, _start_ap as u64, init_ap as u64); /* [todo fix] don't use sbi */
            sbi::sbi_hart_start(i as u64, _start_ap as u64, enter_container as u64); /* [todo fix] don't use sbi */
        }
    }
}
