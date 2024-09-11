//! Virtual M-mode plugin
#![no_std]

pub mod regs;

use regs::vmhartid::Vmhartid;
use regs::vmstatus::Vmstatus;
use regs::vmie::Vmie;
use regs::vmcause::Vmcause;
use regs::vmepc::Vmepc;
use regs::vmtvec::Vmtvec;

extern crate violet;
use violet::library::vm::get_mut_virtual_machine;
use violet::library::vm::VirtualMachine;
use violet::arch::rv64::instruction::csr::Csr;
use violet::arch::rv64::instruction::csr::csrnumber::CsrNumber;
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::rv64::trap::TrapVector;
use violet::arch::rv64::instruction::Instruction;
use violet::arch::rv64::regs::*;
use violet::arch::rv64::instruction::ret::Ret;
use violet::arch::traits::TraitArch;
use violet::arch::traits::hypervisor::HypervisorT;
use violet::environment::Arch;
use core::ptr::write_unaligned;

pub fn init(vm: &mut VirtualMachine) {
    Hext::set_delegation_exc(TrapVector::ILLEGAL_INSTRUCTION);

    // Register virtual machine traps
    if vm.trap.register_traps(
        &[
            (TrapVector::ILLEGAL_INSTRUCTION, do_illegal_instruction),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    // Register virtual machine registers
    match vm.cpu.get_mut(Arch::get_cpuid()) {
        None => {},
        Some(c) => {
            c.register(CsrNumber::Mhartid as usize, Vmhartid::new(c.get_vcpuid() as u64));
            c.register(CsrNumber::Mtvec as usize, Vmtvec::new());
            c.register(CsrNumber::Mstatus as usize, Vmstatus::new());
            c.register(CsrNumber::Mie as usize, Vmie::new());
            c.register(CsrNumber::Mcause as usize, Vmcause::new());
            c.register(CsrNumber::Mepc as usize, Vmepc::new());
        },
    }
}

pub fn do_ecall_from_vsmode(sp: *mut usize) {
    let regs = Registers::from(sp);
    Hext::redirect_to_guest(regs);
}

fn do_illegal_instruction(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_mut_virtual_machine();
    let pepc = vm.mem.get_paddr(regs.epc).unwrap();
    let inst = Instruction::fetch(pepc);
    
    let csr = Csr::from_val(inst);
    match csr {
        /* Not csr access instruction */
        Csr::UNIMP => {
            let ret = Ret::from_val(inst);
            match ret {
                Ret::Mret(_r) => {
                    // replace mret instruction with sret instruction
                    // may be better to emulate mret instruction
                    // There is a possibility that the instruction alignment is not correct due to the compressed instruction
                    // -> use write_unaligned instead of write_volatile
                    unsafe { write_unaligned(pepc as *mut usize, 0x10200073); }
                    return;
                },
                _ => Hext::redirect_to_guest(regs),
            }
        }
        /* Csr access instruction */
        _ => {
            /* read */
            regs.reg[csr.dst()] = vm.cpu.get_mut(Arch::get_cpuid()).unwrap().vregs.get_mut(csr.csr()).unwrap().read() as usize;
            /* write */
            vm.cpu.get_mut(Arch::get_cpuid()).unwrap().vregs.get_mut(csr.csr()).unwrap().write(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
        }
    }
    regs.epc = regs.epc + Instruction::len(inst);
}
