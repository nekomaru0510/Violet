//! ld Instruction

use super::LoadT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Ld {
    inst: LFormat,
}

impl LoadT for Ld {
    fn new(inst: usize) -> Self {
        Ld {
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

impl Ld {
    pub const FUNCT3: usize = 0b011;
    pub const OPCODE: usize = 0b0000011;
}
