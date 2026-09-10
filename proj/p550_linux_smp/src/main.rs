//! Running Linux on virtual machine with violet
#![no_main]
#![no_std]
#![feature(used_with_arg)]
#![feature(riscv_ext_intrinsics)]

extern crate violet;

use core::ptr;
use core::arch::asm;

use csr::Csr;
use violet::arch::rv64::csr::hstatus::{self, Hstatus};
use violet::arch::rv64::csr::mie::STIE;
use violet::library::vm::vdev::vplic::VPlic;
use violet::library::vm::{create_virtual_machine, finalize_virtual_machine, get_virtual_machine, vcpu};
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::rv64::instruction::load::Load;
use violet::arch::rv64::instruction::store::Store;
use violet::arch::rv64::instruction::*;
use violet::arch::rv64::regs::*;
use violet::arch::rv64::sbi;
use violet::arch::rv64::trap::int::Interrupt;
use violet::arch::rv64::trap::TrapVector;
use violet::arch::rv64::vscontext::*;
use violet::arch::traits::context::TraitContext;
use violet::arch::traits::mmu::PageEntryAttribute;
use violet::{environment::*, println};
use violet::arch::rv64::csr::vsatp::Vsatp;
use violet::arch::rv64::csr::sie;
use violet::arch::rv64::csr::sie::*;
use violet::arch::rv64::csr::sip;
use violet::arch::rv64::csr::sip::*;

use violet::arch::rv64::csr::htimedelta::Htimedelta;
use violet::arch::rv64::csr::time::Time;

use violet::kernel::syscall::vsi::create_task;
use violet::resource::{get_resources, BorrowResource, ResourceType};
use violet::print;

use crate::violet::arch::traits::TraitArch;

use violet::app_init;
app_init!(main);

static mut WAKEUP: [bool; NUM_OF_CPUS] = [false; NUM_OF_CPUS];
static mut WAKEUP_ADDR: [usize; NUM_OF_CPUS] = [0; NUM_OF_CPUS];
static mut BOOT_DATA: [usize; NUM_OF_CPUS] = [0; NUM_OF_CPUS];

pub fn do_ecall_from_vsmode(sp: *mut usize) {
    let regs = Registers::from(sp);
    let ext: i32 = regs.reg[A7] as i32;
    let fid: i32 = regs.reg[A6] as i32;

    match sbi::Extension::from_ext(ext) {
        sbi::Extension::SetTimer | sbi::Extension::Timer => {
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
            ));

            Sie::write(sie::STIE, 1);
        }
        sbi::Extension::Ipi => {
            if fid == 0 {
                let vm = get_virtual_machine();
                let vhart_mask = regs.reg[A0] as usize;
                let vhart_mask_base = regs.reg[A1] as usize;
                let (hart_mask, hart_mask_base) = vm.vhart_mask_to_hart_mask(vhart_mask, vhart_mask_base);
                regs.reg[A0] = hart_mask as usize;
                regs.reg[A1] = hart_mask_base as usize;
            } else {
                panic!("Unsupported IPI function id: {}", fid);
            }
        }sbi::Extension::Rfence => {
            match fid {
                // Remote fence.i
                0 => {
                    let vm = get_virtual_machine();
                    let vhart_mask = regs.reg[A0] as usize;
                    let vhart_mask_base= regs.reg[A1] as usize;
                    let (hart_mask, hart_mask_base) = vm.vhart_mask_to_hart_mask(vhart_mask, vhart_mask_base);
                    regs.reg[A0] = hart_mask as usize;
                    regs.reg[A1] = hart_mask_base as usize;
                },
                // Remote sfence.vma
                1 => {
                    let vm = get_virtual_machine();
                    let vhart_mask = regs.reg[A0] as usize;
                    let vhart_mask_base= regs.reg[A1] as usize;
                    let (hart_mask, hart_mask_base) = vm.vhart_mask_to_hart_mask(vhart_mask, vhart_mask_base);
                    let start_addr = regs.reg[A2] as u64;
                    let size = regs.reg[A3] as u64;
                    let (a0, a1) = sbi::sbi_remote_hfence_vvma(hart_mask as u64, hart_mask_base as u64, start_addr, size);
    
                    regs.reg[A0] = a0;
                    regs.reg[A1] = a1;
                    regs.epc = regs.epc + 4;
    
                    return;
                },
                // Remote sfence.vma.asid
                2 => {
                    let vm = get_virtual_machine();
                    let vhart_mask = regs.reg[A0] as usize;
                    let vhart_mask_base= regs.reg[A1] as usize;
                    let (hart_mask, hart_mask_base) = vm.vhart_mask_to_hart_mask(vhart_mask, vhart_mask_base);
                    let asid = regs.reg[A4] as u64;
                    let start_addr = regs.reg[A2] as u64;
                    let size = regs.reg[A3] as u64;
                    let (a0, a1) = sbi::sbi_remote_hfence_vvma_asid(hart_mask as u64, hart_mask_base as u64, start_addr, size, asid);
    
                    regs.reg[A0] = a0;
                    regs.reg[A1] = a1;
                    regs.epc = regs.epc + 4;
    
                    return;
                },
                _ => ()
            }
        }
        sbi::Extension::HartStateManagement => {
            if fid == 0 {
                let vm = get_virtual_machine();
                let pcpuid = vm.get_pcpuid(regs.reg[A0] as usize);

                if pcpuid != usize::MAX {
                    unsafe {
                        WAKEUP_ADDR[pcpuid] = regs.reg[A1];
                        BOOT_DATA[pcpuid] = regs.reg[A2];
                        WAKEUP[pcpuid] = true;
                    }
                }

                regs.reg[A0] = 0;
                regs.reg[A1] = 0;
                regs.epc = regs.epc + 4;

                return;
            }
            // Get status
            if fid == 2 {
                let vm = get_virtual_machine();
                let pcpuid = vm.get_pcpuid(regs.reg[A0] as usize);
                regs.reg[A0] = pcpuid;
            }
        }
        sbi::Extension::SystemReset => loop {},
        _ => {}
    }

    let ret = Instruction::ecall(
        ext,
        fid,
        regs.reg[A0],
        regs.reg[A1],
        regs.reg[A2],
        regs.reg[A3],
        regs.reg[A4],
        regs.reg[A5],
    );

    regs.reg[A0] = ret.0;
    regs.reg[A1] = ret.1;

    regs.epc = regs.epc + 4;
}


/* [todo delete] */
fn topaddr(epc: usize) -> usize {

    // if epc >= 0x1_0000_0000 {
    if Vsatp::get() != 0 {
        (epc & 0x0_ffff_ffff) + 0x8000_0000 + 0x20_0000 //after MMU start in linux
    } else {
        (epc & 0x0_ffff_ffff) + 0x8000_0000 //before MMU start in linux
        // epc
    }
}

pub fn do_guest_store_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    // let inst = Instruction::fetch(topaddr(regs.epc));
    let inst = Instruction::fetch_in_vm(regs.epc);
    let val = Store::from_val(inst).store_value(regs);

    match vm.dev.write().write(fault_paddr, val) {
        None => {
            match vm.map_guest_page(fault_paddr)  {
                Err(_) => {
                    regs.epc = regs.epc + Instruction::len(inst);
                },
                Ok(_) => {
                    vm.set_guest_page_attributes(fault_paddr, &[PageEntryAttribute::Accessed, PageEntryAttribute::Dirty]);
                }
            }
        },
        Some(()) => {
            regs.epc = regs.epc + Instruction::len(inst);
        }
    };
}

pub fn do_guest_load_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    // let inst = Instruction::fetch(topaddr(regs.epc));
    let inst = Instruction::fetch_in_vm(regs.epc);

    match vm.dev.write().read(fault_paddr) {
        None => {
            match vm.map_guest_page(fault_paddr)  {
                Err(_) => {
                    regs.reg[Load::from_val(inst).dst()] = 0;
                    regs.epc = regs.epc + Instruction::len(inst);
                },
                Ok(_) => {
                    vm.set_guest_page_attribute(fault_paddr, PageEntryAttribute::Accessed);
                }
            }
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    };
}

pub fn do_guest_instruction_page_fault(_sp: *mut usize) {
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let _ = vm.map_guest_page(fault_paddr);
    vm.set_guest_page_attribute(fault_paddr, PageEntryAttribute::Accessed);
}

pub fn do_supervisor_external_interrupt(_sp: *mut usize) {    
    let vm = get_virtual_machine();

    if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        let mut  intid = i.get_pend_int();
        while intid != 0 {
            // write to virtual plic
            match vm.dev.write().get_mut(0x0c20_1000) {
                // [todo fix] Make it possible to search by interrupt number
                None => (),
                Some(d) => {
                    d.interrupt(intid as usize);
                }
            }
            intid = i.get_pend_int();
        }
    }
}

pub fn do_supervisor_timer_interrupt(_sp: *mut usize) {
    // Disable the timer
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    // Disable the timer interrupt
    Sie::write(sie::STIE, 0);

    // Raise a timer interrupt to the guest
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
}

pub fn do_supervisor_software_interrupt(_sp: *mut usize) {
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_SOFTWARE_INTERRUPT,
    ));
    // Clear the software interrupt pending bit in sip
    Sip::write(sip::SSIP, 0);
}

pub fn boot_linux_bsp() {
    /* Setup virtual machine */
    create_virtual_machine();
    let vm = get_virtual_machine();
    vm.reset();


    /* CPU */
    {
        let mut vcpu_map = vm.cpu.write();
        vcpu_map.register(0, 0); /* vcpu0 ... pcpu1 */
        vcpu_map.register(1, 1); /* vcpu1 ... pcpu1 */
        vcpu_map.register(2, 2); /* vcpu2 ... pcpu2 */
        vcpu_map.register(3, 3); /* vcpu3 ... pcpu3 */
    }


    /* RAM */
    {
        let mut mem = vm.mem.write();
        // Passthrough
        mem.register(0x00, 0x00, 0x8000_0000); // 0x0_0000_0000 - 0x7_ffff_ffff
        mem.register(0x8000_0000, 0x8000_0000, 0x2_0000_0000); // 0x8000_0000 - 0x2_7fff_ffff (8GiB for guest)
        mem.register(0x10_0000_0000, 0x10_0000_0000, 0x1f0_0000_0000); // 0x10_0000_0000 - 0x1ff_ffff_ffff
    }

    // relocate guest to 0x1_7800_0000
    let src_addr: *const u8 = 0x2_0020_0000 as *const u8;
    let dest_addr: *mut u8 = 0x8020_0000 as *mut u8;
    let size: usize = 0x20_0000;

    unsafe {
        ptr::copy_nonoverlapping(src_addr, dest_addr, size);
    }

    // vm.mem.register(0x8810_0000, 0x9200_0000, 0x20_0000);    // initrd is also mapped to physical memory. The size is estimated from rootfs.img
    vm.mmu_enable();


    /* MMIO */
    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([0, 1, 2, 3]);
    vm.dev.write().register(PLIC_BASE, 0x0400_0000, vplic);

    finalize_virtual_machine();

    if vm.trap.write().register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt),
            (TrapVector::SUPERVISOR_EXTERNAL_INTERRUPT, do_supervisor_external_interrupt),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault),
            (TrapVector::SUPERVISOR_SOFTWARE_INTERRUPT, do_supervisor_software_interrupt),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    match vm.cpu.write().get_mut(0) {
        None => (),
        Some(v) => {
            v.context.set(JUMP_ADDR, 0x8020_0000);
            v.context.set(ARG0, 0);
            v.context.set(ARG1, 0x8220_0000);
        }
    }

    for cpuid in 1..NUM_OF_CPUS {
        create_task(2 + cpuid as u64, boot_linux_ap, cpuid);
    }

    vm.run();
}

pub fn boot_linux_ap() {
    let boot_core = Arch::get_cpuid();
    
    /* Setup virtual machine for each core */
    let vm = get_virtual_machine();
    vm.reset();
    vm.mmu_enable();

    if vm.trap.write().register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt),
            (TrapVector::SUPERVISOR_EXTERNAL_INTERRUPT, do_supervisor_external_interrupt),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault),
            (TrapVector::SUPERVISOR_SOFTWARE_INTERRUPT, do_supervisor_software_interrupt),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    /* Wait for wake up */
    unsafe {while !WAKEUP[boot_core] {}}

    /* Set regs */
    let vcpuid = vm.cpu.read().get_vcpuid();
    match vm.cpu.write().get_mut(vcpuid) {
        None => (),
        Some(v) => {
            v.context.set(JUMP_ADDR, unsafe {WAKEUP_ADDR[boot_core]});
            v.context.set(ARG0, vcpuid);
            v.context.set(ARG1, unsafe {BOOT_DATA[boot_core]});
        }
    }

    /* Run vcpu */
    vm.run();
}

pub fn main() {
    // Boot Linux on core 0
    create_task(2, boot_linux_bsp, 0);
}
