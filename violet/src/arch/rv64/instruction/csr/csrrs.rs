//! csrrs Instruction

use super::CsrT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Csrrs {
    inst: LFormat,
}

impl CsrT for Csrrs {
    fn new(inst: usize) -> Self {
        Csrrs {
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

impl Csrrs {
    pub const FUNCT3: usize = 0b010;
    pub const OPCODE: usize = 0b1110011;
}
