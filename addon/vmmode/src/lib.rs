//! Virtual M-mode plugin
#![no_std]
#![feature(riscv_ext_intrinsics)]

pub mod regs;

use regs::vmhartid::Vmhartid;
use regs::vmstatus::Vmstatus;
use regs::vmie::Vmie;
use regs::vmip::Vmip;
use regs::vmcause::Vmcause;
use regs::vmepc::Vmepc;
use regs::vmtvec::Vmtvec;
use regs::vmideleg::Vmideleg;
use regs::vmedeleg::Vmedeleg;
use regs::vmtval::Vmtval;
use regs::vmscratch::Vmscratch;
use regs::vmisa::Vmisa;

extern crate violet;
use violet::library::vm::get_virtual_machine;
use violet::arch::rv64::instruction::csr::Csr;
use violet::arch::rv64::instruction::csr::csrnumber::CsrNumber;
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::rv64::trap::TrapVector;
use violet::arch::rv64::instruction::Instruction;
use violet::arch::rv64::regs::*;
use violet::arch::rv64::instruction::ret::Ret;
use violet::arch::traits::hypervisor::HypervisorT;
use violet::library::vm::VirtualMachineRef;
use core::ptr::write_unaligned;
use core::arch::riscv64::fence_i;

pub fn init(vm: &VirtualMachineRef) {
    Hext::set_delegation_exc(TrapVector::ILLEGAL_INSTRUCTION);

    // Register virtual machine traps
    if vm.trap.write().register_traps(
        &[
            (TrapVector::ILLEGAL_INSTRUCTION, do_illegal_instruction),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    // Register virtual machine registers
    let vcpuid = vm.cpu.read().get_vcpuid();
    match vm.cpu.write().get_mut(vcpuid) {
        None => {},
        Some(c) => {
            c.register(CsrNumber::Mhartid as usize, Vmhartid::new(c.get_vcpuid() as u64));
            c.register(CsrNumber::Mtvec as usize, Vmtvec::new());
            c.register(CsrNumber::Mstatus as usize, Vmstatus::new());
            c.register(CsrNumber::Mie as usize, Vmie::new());
            c.register(CsrNumber::Mip as usize, Vmip::new());
            c.register(CsrNumber::Mcause as usize, Vmcause::new());
            c.register(CsrNumber::Mepc as usize, Vmepc::new());
            c.register(CsrNumber::Mideleg as usize, Vmideleg::new());
            c.register(CsrNumber::Medeleg as usize, Vmedeleg::new());
            c.register(CsrNumber::Mtval as usize, Vmtval::new());
            c.register(CsrNumber::Mscratch as usize, Vmscratch::new());
            c.register(CsrNumber::Misa as usize, Vmisa::new());
        },
    }
}

pub fn do_ecall_from_vsmode(sp: *mut usize) {
    let regs = Registers::from(sp);
    Hext::redirect_to_guest(regs);
}

fn do_illegal_instruction(sp: *mut usize) {
    let regs = Registers::from(sp);
    let vm = get_virtual_machine();
    let pepc = vm.mem.read().get_paddr(regs.epc).unwrap();
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
                    unsafe {
                        write_unaligned(pepc as *mut usize, 0x10200073);
                        fence_i();
                    }
                    return;
                },
                _ => {
                    Hext::redirect_to_guest(regs);
                    return;
                },
            }
        }
        /* Csr access instruction */
        _ => {
            let vcpuid = vm.cpu.read().get_vcpuid();
            match vm.cpu.write().get_mut(vcpuid).unwrap().vregs.get_mut(csr.csr()) {
                None => {
                    // panic!("Csr access instruction: read: None");
                    Hext::redirect_to_guest(regs);
                    return;
                },
                Some(v) => {
                    let read_val = v.read() as usize;
                    let write_val = csr.write_val(read_val, csr.imm(regs));
                    /* read */
                    regs.reg[csr.dst()] = read_val;
                    /* write */
                    v.write(write_val as u64);
                },
            }
        }
    }
    regs.epc = regs.epc + Instruction::len(inst);
}
