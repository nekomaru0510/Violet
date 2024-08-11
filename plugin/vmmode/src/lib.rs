//! Virtual M-mode plugin
#![no_std]

pub mod regs;

use regs::vmhartid::Vmhartid;
use regs::vmstatus::Vmstatus;
use regs::vmie::Vmie;
use regs::vmcause::Vmcause;
use regs::vmepc::Vmepc;

extern crate violet;
use violet::library::vm::VirtualMachine;
use violet::library::vm::vcpu::vreg::VirtualRegisterT;
use violet::arch::rv64::instruction::csr::Csr;
use violet::arch::rv64::instruction::csr::csrnumber::CsrNumber;
use violet::arch::rv64::extension::hypervisor::Hext;
use violet::arch::rv64::trap::TrapVector;
use violet::arch::rv64::instruction::Instruction;
use violet::arch::rv64::regs::*;
use violet::arch::rv64::redirect_to_guest;
use violet::driver::traits::cpu::hypervisor::HypervisorT;
use violet::environment::cpu_mut;

use violet::arch::rv64::instruction::ret::Ret;
use core::ptr::write_volatile;

pub fn init<H: HypervisorT>(vm: &mut VirtualMachine<H>) {
    Hext::set_delegation_exc(TrapVector::ILLEGAL_INSTRUCTION);
    cpu_mut().register_vector(TrapVector::ILLEGAL_INSTRUCTION, do_illegal_instruction);

    /* 例外ハンドラの登録 */
    cpu_mut().register_vector(
        TrapVector::ENVIRONMENT_CALL_FROM_VSMODE,
        do_ecall_from_vsmode,
    );

    match vm.cpu.get_mut(0) {
        None => {},
        Some(c) => {c.register(CsrNumber::Mhartid as usize, Vmhartid::new(0))},
    }
}

pub fn do_ecall_from_vsmode(sp: *mut usize /*regs: &mut Registers*/) {
    let regs = Registers::from(sp);
    redirect_to_guest(regs);
}

fn do_illegal_instruction(sp: *mut usize) {
    let regs = Registers::from(sp);
    //let inst = Instruction::fetch(regs.epc + 0x4000_0000/*todo delete*/);
    let inst = Instruction::fetch(regs.epc); // デバッグ用
    
    /* CSRアクセス命令 */
    let csr = Csr::from_val(inst);
    match csr {
        Csr::UNIMP => {
            let ret = Ret::from_val(inst);
            match ret {
                Ret::Mret(_r) => {
                    /* mret命令をsret命令に書き換え */
                    /* mret命令をエミュレーションしたほうがいいかも */
                    //unsafe { write_volatile((regs.epc+0x4000_0000)/*todo delete*/ as *mut usize, 0x10200073); }
                    unsafe { write_volatile((regs.epc) as *mut usize, 0x10200073); } /* デバッグ用 */
                    return;
                },
                _ => redirect_to_guest(regs),
            }
        }
        _ => {
            /* CSR番号ごとに処理を実施 */
            match CsrNumber::from_num(csr.csr()) {
                CsrNumber::Mhartid => {
                    /* CSR命令はread->writeを同時に行う */
                    /* vcpu内に仮想レジスタを保存する機能が欲しい */
                    /* csr命令で、read(dstのインデックス)、write(srcの値)の情報を取れるように */
                    /* read */
                    regs.reg[csr.dst()] = 0; /* [todo fix] vmからvcpu番号を取る */
                    /* write */
                }, 
                CsrNumber::Mtvec => {
                    /* mtvecは、vstvecに変更 */
                    /* read */
                    regs.reg[csr.dst()] = Hext::get_vs_vector() as usize;
                    /* write */
                    Hext::set_vs_vector(csr.write_val(regs.reg[csr.dst()], csr.imm(regs)) as u64);
                },
                CsrNumber::Mstatus => {
                    /* mstatusは、vstatusに変更 */
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
