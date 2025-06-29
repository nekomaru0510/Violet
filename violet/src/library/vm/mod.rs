//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! VirtualMachine module provides core virtualization features for the Violet hypervisor.

use alloc::vec::Vec;

pub mod vcpu;
pub mod vdev;
pub mod vmem;
pub mod trap;

use vcpu::VirtualCpuMap;
use vdev::VirtualDevMap;
use vmem::VirtualMemoryMap;
use trap::TrapMap;

use crate::arch::traits::hypervisor::HypervisorT;
use crate::arch::traits::context::TraitContext;
use crate::arch::traits::TraitCpu;
use crate::environment::Arch;
use crate::environment::Hyp;

/// VirtualMachine represents a single virtual machine instance.
pub struct VirtualMachine {
    /// Virtual CPU map for this VM
    pub cpu: VirtualCpuMap,
    /// Virtual memory map for this VM
    pub mem: VirtualMemoryMap,
    /// Virtual device map for this VM
    pub dev: VirtualDevMap,
    /// Trap handler map for this VM
    pub trap: TrapMap,
    /// Virtual machine ID
    pub vm_id: usize,
}

impl VirtualMachine {
    /// Create a new VirtualMachine instance with the given VM ID.
    pub fn new(vm_id: usize) -> VirtualMachine {
        VirtualMachine {
            cpu: VirtualCpuMap::new(),
            mem: VirtualMemoryMap::new(),
            dev: VirtualDevMap::new(),
            trap: TrapMap::new(),
            vm_id,
        }
    }

    /// Reset the virtual machine (default setup before guest boot).
    pub fn reset(&self) {
        // Default setup before guest boot
        Hyp::init();
    }

    /// Run the virtual machine (jump to the current vCPU context).
    pub fn run(&mut self) {
        // Set the VM ID to the current CPU before running
        Arch::get_mut_core().set_vm_id(self.vm_id);
        match self.cpu.get(self.cpu.get_vcpuid()) {
            None => (),
            Some(v) => v.context.jump(),
        };
    }

    /// Enable MMU for the virtual machine.
    pub fn mmu_enable(&self) {
        Hyp::mmu_enable();
    }

    /// Map a guest physical page to the host address space.
    ///
    /// * `guest_paddr` - Guest physical address to map
    pub fn map_guest_page(&mut self, guest_paddr: usize) {
        match self.mem.get(guest_paddr) {
            None => {
                // Pass through addresses that are not set
                // [todo fix] fix size 0x1000
                Hyp::map_vaddr(guest_paddr, guest_paddr, 0x1000);
            }
            Some(m) => {
                match m.get_paddr(guest_paddr) {
                    None => {}
                    Some(r) => {
                        // [todo fix] fix size 0x1000
                        Hyp::map_vaddr(r, guest_paddr, 0x1000);
                    }
                }
            }
        }
    }
}

/* Virtual Machine Table */
/// VirtualMachineTable manages all virtual machines and their mapping to CPU cores.
static mut VIRTUAL_MACHINE_TABLE: VirtualMachineTable = VirtualMachineTable::new();

struct VirtualMachineTable {
    /// List of all virtual machines
    vms: Vec<VirtualMachine>,
}

impl VirtualMachineTable {
    /// Create a new VirtualMachineTable instance.
    pub const fn new() -> Self {
        VirtualMachineTable {
            vms: Vec::new(),
        }
    }

    /// Create a new virtual machine and return its ID.
    pub fn create(&mut self) -> usize {
        let id: usize = self.vms.len();
        self.vms.push(VirtualMachine::new(id));
        id
    }

    /// Get a reference to a virtual machine by ID.
    ///
    /// * `id` - Virtual machine ID
    pub fn get(&self, id: usize) -> &VirtualMachine {
        &self.vms[id]
    }

    /// Get a mutable reference to a virtual machine by ID.
    ///
    /// * `id` - Virtual machine ID
    pub fn get_mut(&mut self, id: usize) -> &mut VirtualMachine {
        &mut self.vms[id]
    }

    /// Get the current virtual machine ID for the running CPU.
    ///
    /// Returns the VM ID mapped to the current physical CPU core.
    pub fn current_id(&self) -> usize {
        current_vm_id()
    }

    /// Check if at least one virtual machine is ready.
    pub fn is_ready(&self) -> bool {
        if self.vms.len() == 0 {
            false
        } else {
            true
        }
    }
}

/* IF function */
/// Create a new virtual machine and return its ID.
pub fn create_virtual_machine() -> usize {
    unsafe { VIRTUAL_MACHINE_TABLE.create() }
}

/// Get a reference to the current virtual machine.
pub fn get_virtual_machine() -> &'static VirtualMachine {
    unsafe { VIRTUAL_MACHINE_TABLE.get(current_vm_id()) }
}

/// Get a mutable reference to the current virtual machine.
pub fn get_mut_virtual_machine() -> &'static mut VirtualMachine {
    unsafe { VIRTUAL_MACHINE_TABLE.get_mut(current_vm_id()) }
}

/// Get the current virtual machine ID for the running CPU.
pub fn current_vm_id() -> usize {
    Arch::get_core().get_vm_id()
}

/// Set the current virtual machine ID for the running CPU.
///
/// # Arguments
/// * `vm_id` - Virtual machine ID to assign to the current physical CPU core
pub fn set_current_vm_id(vm_id: usize) {
    Arch::get_mut_core().set_vm_id(vm_id);
}

/// Check if at least one virtual machine is ready.
pub fn is_ready_virtual_machine() -> bool {
    unsafe { VIRTUAL_MACHINE_TABLE.is_ready() }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test VirtualMachine::new() creates a VM with empty maps
    #[test_case]
    fn test_vm_new() -> Result<(), &'static str> {
        let vm = VirtualMachine::new(0);
        // Check that maps are initialized (not None)
        if vm.cpu.len() != 0 {
            return Err("test_vm_new: cpu map should be empty");
        }
        if vm.mem.len() != 0 {
            return Err("test_vm_new: mem map should be empty");
        }
        if vm.dev.len() != 0 {
            return Err("test_vm_new: dev map should be empty");
        }
        if vm.vm_id != 0 {
            return Err("test_vm_new: vm_id should be 0");
        }
        Ok(())
    }

    /// Test VirtualMachineTable::create() and get()
    #[test_case]
    fn test_vm_table_create_and_get() -> Result<(), &'static str> {
        unsafe {
            let id = VIRTUAL_MACHINE_TABLE.create();
            let vm = VIRTUAL_MACHINE_TABLE.get(id);
            if id != 0 {
                return Err("test_vm_table_create_and_get: first VM id should be 0");
            }
            if vm.cpu.len() != 0 {
                return Err("test_vm_table_create_and_get: cpu map should be empty");
            }
            if vm.vm_id != id {
                return Err("test_vm_table_create_and_get: vm_id should match id");
            }
        }
        Ok(())
    }

    /// Test VirtualMachineTable::is_ready()
    #[test_case]
    fn test_vm_table_is_ready() -> Result<(), &'static str> {
        unsafe {
            // After creation, should be ready
            VIRTUAL_MACHINE_TABLE.create();
            if !VIRTUAL_MACHINE_TABLE.is_ready() {
                return Err("test_vm_table_is_ready: should be ready after create");
            }
        }
        Ok(())
    }

    /// Test VirtualMachineTable::get() with invalid id (should panic or be undefined)
    #[test_case]
    fn test_vm_table_invalid_get() -> Result<(), &'static str> {
        // In no_std, panic cannot be caught, so just document as should panic
        // unsafe { VIRTUAL_MACHINE_TABLE.get(9999); }
        Ok(())
    }

    /// Test VirtualMachine::reset() does not panic
    #[test_case]
    fn test_vm_reset() -> Result<(), &'static str> {
        let vm = VirtualMachine::new(0);
        vm.reset();
        Ok(())
    }

    /// Test VirtualMachine::mmu_enable() does not panic
    #[test_case]
    fn test_vm_mmu_enable() -> Result<(), &'static str> {
        let vm = VirtualMachine::new(0);
        vm.mmu_enable();
        Ok(())
    }

    /// Test VirtualMachine::map_guest_page() with unmapped address
    #[test_case]
    fn test_vm_map_guest_page_unmapped() -> Result<(), &'static str> {
        let mut vm = VirtualMachine::new(0);
        // Should not panic
        vm.map_guest_page(0xdeadbeef);
        Ok(())
    }

    /// Test VirtualMachine::map_guest_page() with mapped address
    #[test_case]
    fn test_vm_map_guest_page_mapped() -> Result<(), &'static str> {
        let mut vm = VirtualMachine::new(0);
        vm.mem.register(0x1000, 0x2000, 0x1000);
        vm.map_guest_page(0x1000);
        Ok(())
    }

    /// Test VirtualMachineTable::current_id() returns 0 for default
    #[test_case]
    fn test_vm_table_current_id() -> Result<(), &'static str> {
        unsafe {
            let id = VIRTUAL_MACHINE_TABLE.current_id();
            if id != 0 {
                return Err("test_vm_table_current_id: default current_id should be 0");
            }
        }
        Ok(())
    }
}
