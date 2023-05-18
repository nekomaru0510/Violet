//! CS Format

use crate::{bit_extract, bit_fill, bitfield};

pub struct CSFormat {
    pub inst: usize,
}

impl CSFormat {
    bitfield!(OP:[1,0]);
    bitfield!(RS2:[4,2]);
    bitfield!(IMM1:[6,5]);
    bitfield!(RS1:[9,7]);
    bitfield!(IMM2:[12,10]);
    bitfield!(FUNCT3:[15,13]);

    pub fn op(&self) -> usize {
        bit_extract!(self.inst, Self::OP)
    }

    pub fn rs2(&self) -> usize {
        bit_extract!(self.inst, Self::RS2)
    }

    pub fn imm1(&self) -> usize {
        bit_extract!(self.inst, Self::IMM1)
    }

    pub fn rs1(&self) -> usize {
        bit_extract!(self.inst, Self::RS1)
    }

    pub fn imm2(&self) -> usize {
        bit_extract!(self.inst, Self::IMM2)
    }

    pub fn funct3(&self) -> usize {
        bit_extract!(self.inst, Self::FUNCT3)
    }
}
