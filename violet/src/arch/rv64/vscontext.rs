//! RISC-V用 ゲストOS用コンテキスト

use super::instruction::Instruction;
use super::regs::Registers;
use super::regs::*;
use crate::arch::rv64::csr::sepc::Sepc;
use crate::arch::rv64::PrivilegeMode;
use crate::arch::rv64::Rv64;
use crate::arch::traits::context::TraitContext;
use crate::arch::traits::registers::TraitRegisters;

pub struct VsContext {
    regs: Registers,
    sepc: usize,
}

impl TraitContext for VsContext {
    type Registers = Registers;

    fn new() -> Self {
        VsContext {
            regs: Registers::new(),
            sepc: 0,
        }
    }

    fn switch(&mut self, regs: &mut Self::Registers) {
        self.regs.switch(regs);

        /* [todo fix] switchする機械語命令(csrrw)があるので、それを使いたい */
        let tmp_epc: usize = Sepc::get() as usize;
        Sepc::set(self.sepc as u64);
        self.sepc = tmp_epc;
    }

    fn set(&mut self, idx: usize, value: usize) {
        if idx < NUM_OF_GP_REGS {
            self.regs.reg[idx] = value;
        } else {
            match idx {
                JUMP_ADDR => {
                    self.sepc = value;
                }
                _ => {}
            }
        }
    }

    fn get(&self, idx: usize) -> usize {
        if idx < NUM_OF_GP_REGS {
            self.regs.reg[idx]
        } else {
            match idx {
                JUMP_ADDR => self.sepc,
                _ => 0,
            }
        }
    }

    fn jump(&self) {
        Rv64::set_next_mode(PrivilegeMode::ModeVS);
        Instruction::sret(self.get(JUMP_ADDR), self.get(ARG0), self.get(ARG1));
    }
}

pub const NUM_OF_GP_REGS: usize = NUM_OF_REGS;
pub const ARG0: usize = A0;
pub const ARG1: usize = A1;
pub const JUMP_ADDR: usize = NUM_OF_GP_REGS + 0;

#[test_case]
fn test_context() -> Result<(), &'static str> {
    let mut c1 = VsContext::new();
    let mut r = Registers::new();

    c1.regs.reg[A0] = 1;
    c1.sepc = 1;

    c1.switch(&mut r);

    if c1.regs.reg[A0] != 0 {
        Err("Fail to switch context 1")
    } else if r.reg[A0] != 1 {
        Err("Fail to switch context 2")
    } else {
        if c1.sepc != 0 {
            Err("Fail to switch context 3")
        } else {
            Ok(())
        }
    }
}

#[test_case]
fn test_context_set() -> Result<(), &'static str> {
    let mut c = VsContext::new();

    c.set(ARG0, 1);
    c.set(JUMP_ADDR, 0x9020_0000);

    if c.regs.reg[A0] != 1 {
        Err("Fail to set context 1")
    } else if c.sepc != 0x9020_0000 {
        Err("Fail to set context 2")
    } else {
        Ok(())
    }
}
