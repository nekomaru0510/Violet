//! R Format

use crate::{bit_extract, bit_fill, bitfield};

pub struct RFormat {
    pub inst: usize,
}

impl RFormat {
    bitfield!(OPCODE:[6,0]);
    bitfield!(RD:[11,7]);
    bitfield!(FUNCT3:[14,12]);
    bitfield!(RS1:[19,15]);
    bitfield!(RS2:[24,20]);
    bitfield!(FUNCT7:[31,25]);

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

    pub fn rs2(&self) -> usize {
        bit_extract!(self.inst, Self::RS2)
    }

    pub fn funct7(&self) -> usize {
        bit_extract!(self.inst, Self::FUNCT7)
    }
}
