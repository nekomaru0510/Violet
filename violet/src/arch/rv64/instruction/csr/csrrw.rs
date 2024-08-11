//! csrrw Instruction

use super::CsrT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Csrrw {
    inst: LFormat,
}

impl CsrT for Csrrw {
    fn new(inst: usize) -> Self {
        Csrrw {
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

impl Csrrw {
    pub const FUNCT3: usize = 0b001;
    pub const OPCODE: usize = 0b1110011;
}
