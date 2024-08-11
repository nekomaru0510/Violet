//! csrrsi Instruction

use super::CsrT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Csrrsi {
    inst: LFormat,
}

impl CsrT for Csrrsi {
    fn new(inst: usize) -> Self {
        Csrrsi {
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

impl Csrrsi {
    pub const FUNCT3: usize = 0b110;
    pub const OPCODE: usize = 0b1110011;
}
