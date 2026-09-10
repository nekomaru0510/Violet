//! Running FreeRTOS on virtual machine with violet
#![allow(static_mut_refs)] /* [todo remove] */

extern crate violet;
extern crate vmmode;

use violet::library::vm::vdev::vclint::VClint;
use violet::library::vm::{create_virtual_machine, get_virtual_machine};

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

pub fn do_guest_store_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(vm.mem.read().get_paddr(regs.epc).unwrap());
    let val = Store::from_val(inst).store_value(regs);

    match vm.dev.write().write(fault_paddr, val) {
        None => {
            let _ = vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(()) => {
            regs.epc = regs.epc + Instruction::len(inst);
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
            ));
        }
    };
}

pub fn do_guest_load_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(vm.mem.read().get_paddr(regs.epc).unwrap());

    match vm.dev.write().read(fault_paddr) {
        None => {
            let _ = vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    };
}

pub fn do_guest_instruction_page_fault(_sp: *mut usize) {
    let vm = get_virtual_machine();
    let _ = vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
}

pub fn do_supervisor_timer_interrupt(_sp: *mut usize) {
    // Disable the timer
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    // Raise a timer interrupt to the guest
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
}

pub fn boot_freertos() {
    /* Setup virtual machine */
    create_virtual_machine();
    let mut vm = get_virtual_machine();
    vm.reset();

    /* CPU */
    {
        let mut vcpu_map = vm.cpu.write();
        vcpu_map.register(0, 0); /* vcpu0 ... pcpu0 */
        match vcpu_map.get_mut(0) {
            None => (),
            Some(v) => {
                v.context.set(JUMP_ADDR, 0x8000_0000);
            }
        }
    } // Drop vcpu_map
    
    /* RAM */
    {
        let mut vmem_map = vm.mem.write();
        vmem_map.register(0x8000_0000, 0xc000_0000, 0x1000_0000);
        // Passthrough
        vmem_map.register(0x00, 0x00, 0x8000_0000);
    } // Drop vmem_map
    
    vm.mmu_enable();

    /* MMIO */
    vm.dev.write().register(0x0200_0000, 0x0001_0000, VClint::new());

    
    /* Register interrupt/exception handler */
    if vm.trap.write().register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }
    
    /* Enable M-mode Virtualization */
    vmmode::init(&vm);

    /* Enter VM */
    vm.run();
}