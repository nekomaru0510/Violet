//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Violet Shell Command for Virtual Machine
//!
//! This module provides a set of vshell commands for outputting Virtual Machine information in Violet.
//! - vm_list: List all virtual machines and their count
//! - vm_vcpu_count: Print the vCPU count for a specified VM
//! - vm_mem_map: Print the memory map for a specified VM

use crate::library::vm::{get_virtual_machine_by_id, get_virtual_machine_count};
use crate::print;
use crate::println;

/// List all virtual machines and print their count and summary.
/// Usage: vm_list
pub fn vm_list(_args: &[&str]) {
    let count = get_virtual_machine_count();
    println!("[VM] count: {}", count);
    for i in 0..count {
        let vm = get_virtual_machine_by_id(i);
        println!("  VM[{}]: vm_id={} vcpu_count={} mem_map_entries={} dev_count={}",
            i, vm.vm_id, vm.cpu.len(), vm.mem.len(), vm.dev.len());
    }
}

/// Print the vCPU count for a specified VM.
/// Usage: vm_vcpu_count <vmid>
pub fn vm_vcpu_count(args: &[&str]) {
    if args.len() < 1 {
        println!("Usage: vm_vcpu_count <vmid>");
        return;
    }
    let vmid = args[0].parse::<usize>().unwrap_or(0);
    let count = get_virtual_machine_count();
    if vmid >= count {
        println!("VM[{}] not found", vmid);
        return;
    }
    let vm = get_virtual_machine_by_id(vmid);
    println!("VM[{}] vCPU count: {}", vmid, vm.cpu.len());
}

/// Print the memory map for a specified VM.
/// Usage: vm_mem_map <vmid>
pub fn vm_mem_map(args: &[&str]) {
    if args.len() < 1 {
        println!("Usage: vm_mem_map <vmid>");
        return;
    }
    let vmid = args[0].parse::<usize>().unwrap_or(0);
    let count = get_virtual_machine_count();
    if vmid >= count {
        println!("VM[{}] not found", vmid);
        return;
    }
    let vm = get_virtual_machine_by_id(vmid);
    println!("VM[{}] mem_map entries: {}", vmid, vm.mem.len());

    for (i, entry) in vm.mem.iter().enumerate() {
        println!("  {}: guest_paddr=0x{:x} host_paddr=0x{:x} size=0x{:x}",
            i, entry.paddr, entry.vaddr, entry.size);
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use super::*;
    use crate::library::vm::{create_virtual_machine, get_virtual_machine_count};

    #[test_case]
    fn test_vm_list_no_vm() -> Result<(), &'static str> {
        // Ensure no VMs exist
        while get_virtual_machine_count() > 0 {
            // No public remove, so just rely on fresh test environment
            break;
        }
        vm_list(&[]); // Should print count: 0
        Ok(())
    }

    #[test_case]
    fn test_vm_list_with_vm() -> Result<(), &'static str> {
        let _ = create_virtual_machine();
        vm_list(&[]); // Should print at least one VM
        Ok(())
    }

    #[test_case]
    fn test_vm_vcpu_count_usage() -> Result<(), &'static str> {
        vm_vcpu_count(&[]); // Should print usage
        Ok(())
    }

    #[test_case]
    fn test_vm_vcpu_count_not_found() -> Result<(), &'static str> {
        let count = get_virtual_machine_count();
        vm_vcpu_count(&[&format!("{}", count)]); // Should print not found
        Ok(())
    }

    #[test_case]
    fn test_vm_mem_map_usage() -> Result<(), &'static str> {
        vm_mem_map(&[]); // Should print usage
        Ok(())
    }

    #[test_case]
    fn test_vm_mem_map_not_found() -> Result<(), &'static str> {
        let count = get_virtual_machine_count();
        vm_mem_map(&[&format!("{}", count)]); // Should print not found
        Ok(())
    }
}




