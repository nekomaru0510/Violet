//! VirtualMachine

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
use crate::arch::traits::TraitArch;
use crate::environment::Arch;
use crate::environment::Hyp;
use crate::environment::NUM_OF_CPUS;

pub struct VirtualMachine {
    pub cpu: VirtualCpuMap,
    pub mem: VirtualMemoryMap,
    pub dev: VirtualDevMap,
    pub trap: TrapMap,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            cpu: VirtualCpuMap::new(),
            mem: VirtualMemoryMap::new(),
            dev: VirtualDevMap::new(),
            trap: TrapMap::new(),
        }
    }

    pub fn reset(&self) {
        // Default setup before guest boot
        Hyp::init();
    }

    pub fn run(&mut self) {
        match self.cpu.get(self.cpu.get_vcpuid()) {
            None => (),
            Some(v) => v.context.jump(),
        };
    }

    pub fn mmu_enable(&self) {
        Hyp::mmu_enable();
    }

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
static mut VIRTUAL_MACHINE_TABLE: VirtualMachineTable = VirtualMachineTable::new();

struct VirtualMachineTable {
    vms: Vec<VirtualMachine>,
    cpu2vm: [usize; NUM_OF_CPUS],
}

impl VirtualMachineTable {
    pub const fn new() -> Self {
        VirtualMachineTable {
            vms: Vec::new(),
            cpu2vm: [0; NUM_OF_CPUS],
        }
    }

    pub fn create(&mut self) -> usize {
        let id: usize = self.vms.len();
        self.vms.push(VirtualMachine::new());
        id
    }

    pub fn get(&self, id: usize) -> &VirtualMachine {
        &self.vms[id]
    }

    pub fn get_mut(&mut self, id: usize) -> &mut VirtualMachine {
        &mut self.vms[id]
    }

    pub fn current_id(&self) -> usize {
        self.cpu2vm[Arch::get_cpuid()]
    }

    pub fn is_ready(&self) -> bool {
        if self.vms.len() == 0 {
            false
        } else {
            true
        }
    }
}

/* IF function */
pub fn create_virtual_machine() -> usize {
    unsafe { VIRTUAL_MACHINE_TABLE.create() }
}

pub fn get_virtual_machine() -> &'static VirtualMachine {
    unsafe { VIRTUAL_MACHINE_TABLE.get(current_vm_id()) }
}

pub fn get_mut_virtual_machine() -> &'static mut VirtualMachine {
    unsafe { VIRTUAL_MACHINE_TABLE.get_mut(current_vm_id()) }
}

pub fn current_vm_id() -> usize {
    unsafe { VIRTUAL_MACHINE_TABLE.current_id() }
}

pub fn is_ready_virtual_machine() -> bool {
    unsafe { VIRTUAL_MACHINE_TABLE.is_ready() }
}

#[cfg(test)]
use crate::library::vm::vdev::vplic::VPlic;

#[test_case]
fn test_read_write_dev() -> Result<(), &'static str> {
    let mut vm: VirtualMachine = VirtualMachine::new();
    let vplic = VPlic::new();
    let val = 0x01;
    vm.dev.register(0x0c00_0000, 0x0400_0000, vplic);

    let mut result = match vm.dev.write(0xc00_0000, val) {
        None => Err("can't write virtual device"),
        Some(x) => Ok(()),
    };
    if result != Ok(()) {
        return result;
    };

    result = match vm.dev.read(0xc00_0000) {
        None => Err("can't read virtual device"),
        Some(x) => {
            if x == val {
                Ok(())
            } else {
                Err("Invalid value")
            }
        }
    };

    result
}

#[cfg(test)]
use crate::arch::rv64::vscontext::*; //[todo delete]

#[test_case]
fn test_vcpu() -> Result<(), &'static str> {
    let mut vm: VirtualMachine = VirtualMachine::new();
    vm.cpu.register(1, 0);
    match vm.cpu.get_mut(1) {
        None => (),
        Some(v) => {
            v.context.set(JUMP_ADDR /*EPC*/, 0x9020_0000);
            v.context.set(ARG0, 0x0000_0000);
            v.context.set(ARG1, 0x0000_0000);
        }
    }
    //vm.run();

    Ok(())
}

#[test_case]
fn test_vmem() -> Result<(), &'static str> {
    let mut vm: VirtualMachine = VirtualMachine::new();
    
    vm.mem.register(0x8020_0000, 0x9020_0000, 0x1000_0000);
    vm.mem.register(0x8220_0000, 0x8220_0000, 0x2_0000);
    vm.mem.register(0x8810_0000, 0x88100000, 0x20_0000);

    Ok(())
}