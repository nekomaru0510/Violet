//! lh Instruction

use super::LoadT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Lh {
    inst: LFormat,
}

impl LoadT for Lh {
    fn new(inst: usize) -> Self {
        Lh {
            inst: LFormat { inst },
        }
    }

    fn rd(&self) -> usize {
        self.inst.rd()
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn offset(&self) -> usize {
        self.inst.imm()
    }
}

impl Lh {
    pub const FUNCT3: usize = 0b001;
    pub const OPCODE: usize = 0b0000011;
}
