//! Running FreeRTOS on virtual machine with violet

#![no_main]
#![no_std]
#![feature(used_with_arg)]
#![allow(static_mut_refs)] /* [todo remove] */

extern crate violet;
extern crate vmmode;

use violet::library::vm::vdev::vclint::VClint;
use violet::library::vm::{create_virtual_machine, get_mut_virtual_machine};

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

use violet::app_init;
app_init!(main);

pub fn do_guest_store_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_mut_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(vm.mem.get_paddr(regs.epc).unwrap());
    let val = Store::from_val(inst).store_value(regs);

    match vm.dev.write(fault_paddr, val) {
        None => {
            vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(()) => {
            regs.epc = regs.epc + Instruction::len(inst);
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
            ));
        }
    }
}

pub fn do_guest_load_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_mut_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(vm.mem.get_paddr(regs.epc).unwrap());

    match vm.dev.read(fault_paddr) {
        None => {
            vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    }
}

pub fn do_guest_instruction_page_fault(_sp: *mut usize) {
    let vm = get_mut_virtual_machine();
    vm.map_guest_page(Hext::get_vs_fault_paddr() as usize);
}

pub fn do_supervisor_timer_interrupt(_sp: *mut usize) {
    // Disable the timer
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    // Raise a timer interrupt to the guest
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
}

fn boot_freertos() {

    create_virtual_machine();
    let mut vm = get_mut_virtual_machine();

    let vclint = VClint::new();

    /* CPU */
    vm.cpu.register(0, 0); /* vcpu0 ... pcpu0 */
    match vm.cpu.get_mut(0) {
        None => (),
        Some(v) => {
            v.context.set(JUMP_ADDR, 0xC000_0000);
        }
    }
    
    /* RAM */
    //vm.mem.register(0x8000_0000, 0xc000_0000, 0x1000_0000);
    vm.mem.register(0xc000_0000, 0xc000_0000, 0x1000_0000);

    /* MMIO */
    vm.dev.register(0x0200_0000, 0x0001_0000, vclint);

    vm.setup();

    /* Enable Interrupt */
    Interrupt::enable_mask_s(
        Interrupt::bit(Interrupt::SUPERVISOR_TIMER_INTERRUPT)
        | Interrupt::bit(Interrupt::SUPERVISOR_EXTERNAL_INTERRUPT),
    );
    
    /* Register interrupt/exception handler */
    if vm.trap.register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }
    
    vmmode::init(&mut vm);
    //vmmode::init(addr_of_mut!(vm));

    vm.run();
}

pub fn main() {
    boot_freertos();
}
