//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Running Linux on virtual machine with violet
#![no_main]
#![no_std]
#![feature(used_with_arg)]

pub mod setup;

extern crate violet;

use violet::library::vm::vdev::vplic::VPlic;
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

use violet::kernel::syscall::vsi::create_task;
use violet::environment::resource::{get_resources, BorrowResource, ResourceType};

pub fn do_ecall_from_vsmode(sp: *mut usize) {
    let regs = Registers::from(sp);
    let ext: i32 = regs.reg[A7] as i32;
    let fid: i32 = regs.reg[A6] as i32;

    match sbi::Extension::from_ext(ext) {
        sbi::Extension::SetTimer | sbi::Extension::Timer => {
            Hext::flush_vsmode_interrupt(Interrupt::bit(
                Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
            ));
        }
        sbi::Extension::HartStateManagement => {
            if fid == 0 {
                regs.reg[A0] = 0;
                regs.reg[A1] = 0;
                regs.epc = regs.epc + 4;

                return;
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
    if epc >= 0x1_0000_0000 {
        (epc & 0x0_ffff_ffff) + 0x1000_0000 + 0x20_0000 //after MMU start in linux
    } else {
        (epc & 0x0_ffff_ffff) + 0x1000_0000 //before MMU start in linux
    }
}

pub fn do_guest_store_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_mut_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(topaddr(regs.epc));
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
    let inst = Instruction::fetch(topaddr(regs.epc));

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

pub fn do_supervisor_external_interrupt(_sp: *mut usize) {
    let vm = get_mut_virtual_machine();
    // Read and clear the pending bit from the physical PLIC
    let int_id = if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        i.get_pend_int()
    } else {
        0
    };

    // write to virtual plic
    match vm.dev.get_mut(0x0c20_1000) {
        // [todo fix] Make it possible to search by interrupt number
        None => (),
        Some(d) => {
            d.interrupt(int_id as usize);
        }
    }

    // Raise a virtual external interrupt 
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_EXTERNAL_INTERRUPT,
    ));

    // Clear the pending bit in the PLIC 
    if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        i.set_comp_int(int_id);
    }
}

pub fn do_supervisor_timer_interrupt(_sp: *mut usize) {
    // Disable the timer
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    // Raise a timer interrupt to the guest
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
}

pub fn boot_linux() {
    let boot_core = 1;
    
    /* Setup virtual machine */
    create_virtual_machine();
    let vm = get_mut_virtual_machine();
    vm.reset();

    /* CPU */
    vm.cpu.register(0, boot_core); /* vcpu0 ... pcpu1 */
    match vm.cpu.get_mut(0) {
        None => (),
        Some(v) => {
            v.context.set(JUMP_ADDR, 0x8020_0000);
            v.context.set(ARG0, 0);
            v.context.set(ARG1, 0x8220_0000);
        }
    }

    /* RAM */
    vm.mem.register(0x8020_0000, 0x9020_0000, 0x1000_0000);
    vm.mem.register(0x8220_0000, 0x8220_0000, 0x2_0000);    // FDT is mapped to physical memory.
    vm.mem.register(0x8810_0000, 0x8810_0000, 0x20_0000);    // initrd is also mapped to physical memory. The size is estimated from rootfs.img
    vm.mmu_enable();

    /* MMIO */
    let mut vplic = VPlic::new();
    vplic.set_vcpu_config([boot_core, 0]); /* vcpu0 ... pcpu1 */
    vm.dev.register(0x0c00_0000, 0x0400_0000, vplic);
    
    /* Register interrupt/exception handler */
    if vm.trap.register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt),
            (TrapVector::SUPERVISOR_EXTERNAL_INTERRUPT, do_supervisor_external_interrupt),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    /* Run */
    vm.run();
}

pub fn main() {
    // Boot Linux on core 1
    create_task(2, boot_linux, 1);
}
