//! lw Instruction

use super::LoadT;
use crate::driver::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Lw {
    inst: LFormat,
}

impl LoadT for Lw {
    fn new(inst: usize) -> Self {
        Lw {
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

impl Lw {
    pub const FUNCT3: usize = 0b010;
    pub const OPCODE: usize = 0b0000011;
}
