//! Running bench on virtual machine with violet

#![no_main]
#![no_std]
#![feature(used_with_arg)]
#![allow(static_mut_refs)] /* [todo remove] */
#![feature(riscv_ext_intrinsics)]

extern crate violet;

use violet::library::vm::get_virtual_machine;
use violet::library::vm::vdev::vclint::VClint;
use violet::library::vm::create_virtual_machine;

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
use violet::environment::*;

use violet::arch::rv64::csr::sie::Sie;
use violet::arch::rv64::csr::sie;

use violet::app_init;
app_init!(main);


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

    let mut ret = Instruction::ecall(
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

pub fn do_guest_store_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(vm.mem.read().get_paddr(regs.epc).unwrap());
    let val = Store::from_val(inst).store_value(regs);

    match vm.dev.write().write(fault_paddr, val) {
        None => {
            vm.map_guest_page(fault_paddr);
            vm.set_guest_page_attributes(fault_paddr, &[PageEntryAttribute::Accessed, PageEntryAttribute::Dirty]);
        }
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
            vm.map_guest_page(fault_paddr);
            vm.set_guest_page_attribute(fault_paddr, PageEntryAttribute::Accessed);
        }
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    };
}

pub fn do_guest_instruction_page_fault(_sp: *mut usize) {
    let vm = get_virtual_machine();
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    vm.map_guest_page(fault_paddr);
    vm.set_guest_page_attribute(fault_paddr, PageEntryAttribute::Accessed);
}

pub fn do_supervisor_timer_interrupt(_sp: *mut usize) {
    // Disable the timer
    sbi::sbi_set_timer(0xffff_ffff_ffff_ffff);

    // Raise a timer interrupt to the guest
    Hext::assert_vsmode_interrupt(Interrupt::bit(
        Interrupt::VIRTUAL_SUPERVISOR_TIMER_INTERRUPT,
    ));
}

fn boot_bench() {
    /* Setup virtual machine */
    create_virtual_machine();
    let mut vm = get_virtual_machine();
    vm.reset();

    /* CPU */
    {
        let mut cpu_map = vm.cpu.write();
        cpu_map.register(0, 0); /* vcpu0 ... pcpu0 */
        match cpu_map.get_mut(0) {
            None => (),
            Some(v) => {
                v.context.set(JUMP_ADDR, 0x8020_0000);
            }
        }
    }

    /* RAM */
    {
        let mut mem_map = vm.mem.write();
        mem_map.register(0x8020_0000, 0xc000_0000, 0x1000_0000);
        // Passthrough
        mem_map.register(0x00, 0x00, 0x8000_0000);
    }
    vm.mmu_enable();

    
    /* Register interrupt/exception handler */
    if vm.trap.write().register_traps(
        &[
            (TrapVector::SUPERVISOR_TIMER_INTERRUPT, do_supervisor_timer_interrupt),
            (TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault),
            (TrapVector::STORE_AMO_GUEST_PAGE_FAULT, do_guest_store_page_fault),
            (TrapVector::INSTRUCTION_GUEST_PAGE_FAULT, do_guest_instruction_page_fault),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    /* Enter VM */
    vm.run();
}

pub fn main() {
    boot_bench();
}
