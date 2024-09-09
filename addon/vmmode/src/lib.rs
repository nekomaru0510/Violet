//! Virtual M-mode plugin
#![no_std]

pub mod regs;

use regs::vmhartid::Vmhartid;
use regs::vmstatus::Vmstatus;
use regs::vmie::Vmie;
use regs::vmcause::Vmcause;
use regs::vmepc::Vmepc;

extern crate violet;
use violet::library::vm::get_mut_virtual_machine;
use violet::library::vm::VirtualMachine;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::instruction::csr::Csr;
use violet::arch::rv64::instruction::csr::csrnumber::CsrNumber;
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::rv64::trap::TrapVector;
use violet::arch::rv64::instruction::Instruction;
use violet::arch::rv64::regs::*;
use violet::arch::rv64::instruction::ret::Ret;
use violet::arch::traits::hypervisor::HypervisorT;
use violet::environment::Hyp;
use core::ptr::write_unaligned;

pub fn init(vm: &mut VirtualMachine) {
    Hext::set_delegation_exc(TrapVector::ILLEGAL_INSTRUCTION);

    if vm.trap.register_traps(
        &[
            (TrapVector::ILLEGAL_INSTRUCTION, do_illegal_instruction),
            (TrapVector::ENVIRONMENT_CALL_FROM_VSMODE, do_ecall_from_vsmode),
        ]
    ) == Err(()) { panic!("Fail to register trap"); }

    match vm.cpu.get_mut(0) {
        None => {},
        Some(c) => {c.register(CsrNumber::Mhartid as usize, Vmhartid::new(0))},
    }
}

pub fn do_ecall_from_vsmode(sp: *mut usize) {
    let regs = Registers::from(sp);
    Hyp::redirect_to_guest(regs);
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
                _ => Hyp::redirect_to_guest(regs),
            }
        }
        /* Csr access instruction */
        _ => {
            /* [todo fix] use vm.vregs */
            match CsrNumber::from_num(csr.csr()) {
                CsrNumber::Mhartid => {
                    /* read */
                    regs.reg[csr.dst()] = vm.cpu.get_vcpuid();
                    /* write */
                }, 
                CsrNumber::Mtvec => {
                    /* read */
                    regs.reg[csr.dst()] = Hext::get_vs_vector() as usize;
                    /* write */
                    Hext::set_vs_vector(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
                },
                CsrNumber::Mstatus => {
                    let mut vmstatus = Vmstatus::new();
                    /* read */
                    regs.reg[csr.dst()] = vmstatus.read() as usize;
                    /* write */
                    vmstatus.write(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
                },
                CsrNumber::Mie => {
                    let mut vmie = Vmie::new();
                    /* read */
                    regs.reg[csr.dst()] = vmie.read() as usize;
                    /* write */
                    vmie.write(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
                },
                CsrNumber::Mcause => {
                    let mut vmcause = Vmcause::new();
                    /* read */
                    regs.reg[csr.dst()] = vmcause.read() as usize;
                    /* write */
                    vmcause.write(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
                },
                CsrNumber::Mepc => {
                    let mut vmepc = Vmepc::new();
                    /* read */
                    regs.reg[csr.dst()] = vmepc.read() as usize;
                    /* write */
                    vmepc.write(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
                },
                _ => {
                    panic!("Unimplemented Csr Virtualization"); 
                },
            }
        }
    }
    regs.epc = regs.epc + Instruction::len(inst);
}
