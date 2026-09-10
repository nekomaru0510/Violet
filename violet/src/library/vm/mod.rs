//! VirtualMachine
use core::arch::riscv64::hfence_gvma_gaddr;

use alloc::sync::Arc;
use alloc::vec::Vec;

pub mod trap;
pub mod vcpu;
pub mod vdev;
pub mod vmem;

use spin::RwLock;
use vcpu::VirtualCpuMap;
use vdev::VirtualDevMap;
use vmem::VirtualMemoryMap;

use crate::library::vm::trap::TrapMap;

use crate::arch::rv64::mmu::get_new_root_page_table_addr_x4;
use crate::arch::rv64::PagingMode;
use crate::arch::traits::hypervisor::HypervisorT;
use crate::arch::traits::context::TraitContext;
use crate::arch::traits::mmu::PageEntryAttribute;
use crate::arch::traits::TraitArch;
use crate::environment::Arch;
use crate::environment::Hyp;
use crate::environment::NUM_OF_CPUS;

pub struct VirtualMachine {
    pub cpu: RwLock<VirtualCpuMap>,
    pub mem: RwLock<VirtualMemoryMap>,
    pub dev: RwLock<VirtualDevMap>,
    pub trap: RwLock<TrapMap>,
    pub root_table: RwLock<Option<usize>>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            cpu: RwLock::new(VirtualCpuMap::new()),
            mem: RwLock::new(VirtualMemoryMap::new()),
            dev: RwLock::new(VirtualDevMap::new()),
            trap: RwLock::new(TrapMap::new()),
            root_table: RwLock::new(None),
        }
    }

    pub fn reset(&self) {
        // Default setup before guest boot
        Hyp::init();
        Hyp::reset();
    }

    pub fn run(&self) {
        let vcpuid = self.cpu.read().get_vcpuid();
        let context = match self.cpu.read().get(vcpuid) {
            None => None,
            Some(v) => Some(v.context.clone()),
        };
        match context {
            None => {
                // If no context is set, we cannot run the VM
                panic!("No context set for vcpu {}", vcpuid);
            }
            Some(ctx) => ctx.jump(),
        }
    }

    pub fn mmu_enable(&self) {
        // Hyp::mmu_enable();
        let mut root_table = self.root_table.write();
        if root_table.is_none() {
            *root_table = Some(get_new_root_page_table_addr_x4());
        }
        Hyp::set_table_addr(root_table.unwrap());
        Hyp::set_paging_mode(PagingMode::Sv48x4);
    }

    pub fn map_guest_page(&self, guest_paddr: usize) -> Result<(), &'static str> {
        match self.mem.read().get(guest_paddr) {
            None => {
                // Pass through addresses that are not set (Disabled now)
                // [todo fix] fix size 0x1000
                // Hyp::map_vaddr(guest_paddr, guest_paddr, 0x1000);
                Err("Invalid guest physical address")
            }
            Some(m) => {
                match m.get_paddr(guest_paddr) {
                    None => {Ok(())}
                    Some(r) => {
                        // [todo fix] fix size 0x1000
                        Hyp::map_vaddr(r, guest_paddr, 0x1000);
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn set_guest_page_attribute(&self, guest_paddr: usize, attr: PageEntryAttribute) {
        Hyp::set_attribute(guest_paddr, attr);
    }

    pub fn set_guest_page_attributes(&self, guest_paddr: usize, attrs: &[PageEntryAttribute]) {
        for attr in attrs {
            Hyp::set_attribute(guest_paddr, *attr);
        }
        // Flush TLB
        unsafe { hfence_gvma_gaddr(guest_paddr >> 2) };
    }
    
    pub fn get_pcpuid(&self, vcpuid: usize) -> usize {
        self.cpu.read().get_pcpuid(vcpuid)
    }

    /* Return hart_mask, hart_mask_base from vhart_mask, vhart_mask_base */
    // [todo fix] limited CPU nums to usize::MAX + usize::BITS
    pub fn vhart_mask_to_hart_mask(&self, vhart_mask: usize, vhart_mask_base: usize) -> (usize, usize) {
        if vhart_mask_base == usize::MAX {
            return (0, usize::MAX);
        }
        let mut hart_mask: usize = 0;
        let vhart_mask = vhart_mask << vhart_mask_base;

        for i in 0..self.cpu.read().get_vcpu_count() {
            if (vhart_mask & (1 << i)) > 0 {
                hart_mask |= 1 << self.get_pcpuid(i);
            }
        }
        (hart_mask, 0)
    }
}

/* Virtual Machine Table */
static mut VIRTUAL_MACHINE_TABLE: RwLock<VirtualMachineTable> = RwLock::new(VirtualMachineTable::new());

pub type VirtualMachineRef = Arc<VirtualMachine>;

struct VirtualMachineTable {
    vms: Vec<VirtualMachineRef>,
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
        self.vms.push(Arc::new(VirtualMachine::new()));
        self.cpu2vm[Arch::get_cpuid()] = id;
        id
    }

    pub fn get(&self, id: usize) -> VirtualMachineRef {
        self.vms[id].clone()
    }

    // pub fn get_mut(&mut self, id: usize) -> &mut VirtualMachine {
    //     &mut self.vms[id]
    // }

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

    fn update_cpu2vm(&mut self, id: usize) {
        let vm = &self.vms[id];
        let vcpu_map = vm.cpu.read();
        for i in 0..vcpu_map.get_vcpu_count() {
            let pcpu_id = vcpu_map.get_pcpuid(i);
            self.cpu2vm[pcpu_id] = id;
        }
    }
}

/* IF function */
pub fn create_virtual_machine() -> usize {
    unsafe { VIRTUAL_MACHINE_TABLE.write().create() }
}

pub fn get_virtual_machine() -> VirtualMachineRef {
    let vm_id = current_vm_id();
    unsafe { VIRTUAL_MACHINE_TABLE.read().get(vm_id) }
}

// pub fn get_virtual_machine() -> &'static mut VirtualMachine {
//     unsafe { VIRTUAL_MACHINE_TABLE.get_mut().get_mut(current_vm_id()) }
// }

pub fn current_vm_id() -> usize {
    unsafe { VIRTUAL_MACHINE_TABLE.read().current_id() }
}

pub fn is_ready_virtual_machine() -> bool {
    unsafe { VIRTUAL_MACHINE_TABLE.read().is_ready() }
}

/* Finalizes the virtual machine setup, ensuring it is ready for execution */
pub fn finalize_virtual_machine()  {
    let vm_id = current_vm_id();
    unsafe { VIRTUAL_MACHINE_TABLE.write().update_cpu2vm(vm_id) }
}

#[cfg(test)]
use crate::library::vm::vdev::vplic::VPlic;

#[test_case]
fn test_read_write_dev() -> Result<(), &'static str> {
    let vm: VirtualMachine = VirtualMachine::new();
    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([0, 1]);
    let val = 0x01;

    let mut vdev_map = vm.dev.write();

    vdev_map.register(0x0c00_0000, 0x0400_0000, vplic);

    let mut result = match vdev_map.write(0xc00_0000, val) {
        None => Err("can't write virtual device"),
        Some(x) => Ok(()),
    };
    if result != Ok(()) {
        return result;
    };

    result = match vdev_map.read(0xc00_0000) {
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
    let vm: VirtualMachine = VirtualMachine::new();
    {
        let mut vcpu = vm.cpu.write();
        vcpu.register(1, 0);
        match vcpu.get_mut(1) {
            None => (),
            Some(v) => {
                v.context.set(JUMP_ADDR /*EPC*/, 0x9020_0000);
                v.context.set(ARG0, 0x0000_0000);
                v.context.set(ARG1, 0x0000_0000);
            }
        }
    }
    //vm.run();

    Ok(())
}

#[test_case]
fn test_vmem() -> Result<(), &'static str> {
    let vm: VirtualMachine = VirtualMachine::new();
    let mut vmem = vm.mem.write();
    
    vmem.register(0x8020_0000, 0x9020_0000, 0x1000_0000);
    vmem.register(0x8220_0000, 0x8220_0000, 0x2_0000);
    vmem.register(0x8810_0000, 0x88100000, 0x20_0000);

    Ok(())
}