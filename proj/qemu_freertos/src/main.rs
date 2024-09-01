//! Running FreeRTOS on virtual machine with violet

#![no_main]
#![no_std]
#![feature(used_with_arg)]
#![allow(static_mut_refs)] /* [todo remove] */

extern crate violet;
extern crate vmmode;
use violet::environment::cpu_mut;

use violet::library::vm::vdev::vclint::VClint;
use violet::library::vm::VirtualMachine;

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

use violet::resource::{get_resources, BorrowResource, ResourceType};

use violet::app_init;
app_init!(main);

static mut VM: VirtualMachine = VirtualMachine::new();

pub fn do_guest_store_page_fault(sp: *mut usize) {
    let regs = Registers::from(sp);
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;

    let inst = Instruction::fetch(unsafe {VM.mem.get_paddr(regs.epc).unwrap()});
    let val = Store::from_val(inst).store_value(regs);

    match unsafe { VM.dev.write(fault_paddr, val) } {
        None => unsafe {
            VM.map_guest_page(Hext::get_vs_fault_paddr() as usize);
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
    let fault_paddr = Hext::get_vs_fault_paddr() as usize;
    let inst = Instruction::fetch(unsafe {VM.mem.get_paddr(regs.epc).unwrap()});

    match unsafe { VM.dev.read(fault_paddr) } {
        None => unsafe {
            VM.map_guest_page(Hext::get_vs_fault_paddr() as usize);
        },
        Some(x) => {
            regs.reg[Load::from_val(inst).dst()] = x;
            regs.epc = regs.epc + Instruction::len(inst);
        }
    }
}

pub fn do_guest_instruction_page_fault(_sp: *mut usize) {
    unsafe {
        VM.map_guest_page(Hext::get_vs_fault_paddr() as usize);
    }
}

pub fn do_supervisor_external_interrupt(_sp: *mut usize) {
    // Read and clear the pending bit from the physical PLIC
    let int_id = if let BorrowResource::Intc(i) = get_resources().get(ResourceType::Intc, 0) {
        i.get_pend_int()
    } else {
        0
    };

    // write to virtual plic
    unsafe {
        match VM.dev.get_mut(0x0c20_1000) {
            // [todo fix] Make it possible to search by interrupt number
            None => (),
            Some(d) => {
                d.interrupt(int_id as usize);
            }
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

fn boot_freertos() {
    let vclint = VClint::new();
    unsafe {
        /* CPU */
        VM.cpu.register(0, 0); /* vcpu0 ... pcpu0 */
        match VM.cpu.get_mut(0) {
            None => (),
            Some(v) => {
                v.context.set(JUMP_ADDR, 0xC000_0000);
            }
        }
        
        /* RAM */
        //VM.mem.register(0x8000_0000, 0xc000_0000, 0x1000_0000);
        VM.mem.register(0xc000_0000, 0xc000_0000, 0x1000_0000);

        /* MMIO */
        VM.dev.register(0x0200_0000, 0x0001_0000, vclint);
    }

    unsafe {
        VM.setup();
    }

    /* Enable Interrupt */
    Interrupt::enable_mask_s(
        Interrupt::bit(Interrupt::SUPERVISOR_TIMER_INTERRUPT)
        | Interrupt::bit(Interrupt::SUPERVISOR_EXTERNAL_INTERRUPT),
    );
    
    cpu_mut().register_vector(
        TrapVector::SUPERVISOR_TIMER_INTERRUPT,
        do_supervisor_timer_interrupt,
    );
    cpu_mut().register_vector(TrapVector::LOAD_GUEST_PAGE_FAULT, do_guest_load_page_fault);
    cpu_mut().register_vector(
        TrapVector::STORE_AMO_GUEST_PAGE_FAULT,
        do_guest_store_page_fault,
    );
    cpu_mut().register_vector(
        TrapVector::INSTRUCTION_GUEST_PAGE_FAULT,
        do_guest_instruction_page_fault,
    );
    
    unsafe {vmmode::init(&mut VM);}
    //unsafe {vmmode::init(addr_of_mut!(VM));}

    unsafe {
        VM.run();
    }
}

pub fn main() {
    boot_freertos();
}
