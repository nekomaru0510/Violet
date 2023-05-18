//! L Format

use crate::{bit_extract, bit_fill, bitfield};

pub struct LFormat {
    pub inst: usize,
}

impl LFormat {
    bitfield!(OPCODE:[6,0]);
    bitfield!(RD:[11,7]);
    bitfield!(FUNCT3:[14,12]);
    bitfield!(RS1:[19,15]);
    bitfield!(IMM:[31,20]);

    pub fn opcode(&self) -> usize {
        bit_extract!(self.inst, Self::OPCODE)
    }

    pub fn rd(&self) -> usize {
        bit_extract!(self.inst, Self::RD)
    }

    pub fn funct3(&self) -> usize {
        bit_extract!(self.inst, Self::FUNCT3)
    }

    pub fn rs1(&self) -> usize {
        bit_extract!(self.inst, Self::RS1)
    }

    pub fn imm(&self) -> usize {
        bit_extract!(self.inst, Self::IMM)
    }
}
