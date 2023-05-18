//! csrrc Instruction

use super::CsrT;
use crate::driver::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Csrrc {
    inst: LFormat,
}

impl CsrT for Csrrc {
    fn new(inst: usize) -> Self {
        Csrrc {
            inst: LFormat { inst },
        }
    }

    fn rd(&self) -> usize {
        self.inst.rd()
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn zimm(&self) -> usize {
        self.inst.rs1()
    }

    fn csr(&self) -> usize {
        self.inst.imm()
    }
}

impl Csrrc {
    pub const FUNCT3: usize = 0b011;
    pub const OPCODE: usize = 0b1110011;
}
