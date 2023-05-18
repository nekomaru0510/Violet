//! S Format

use crate::{bit_extract, bit_fill, bitfield};

pub struct SFormat {
    pub inst: usize,
}

impl SFormat {
    bitfield!(OPCODE:[6,0]);
    bitfield!(IMM1:[11,7]);
    bitfield!(FUNCT3:[14,12]);
    bitfield!(RS1:[19,15]);
    bitfield!(RS2:[24,20]);
    bitfield!(IMM2:[31,25]);

    pub fn opcode(&self) -> usize {
        bit_extract!(self.inst, Self::OPCODE)
    }

    pub fn imm1(&self) -> usize {
        bit_extract!(self.inst, Self::IMM1)
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

    pub fn imm2(&self) -> usize {
        bit_extract!(self.inst, Self::IMM2)
    }

    pub fn imm(&self) -> usize {
        /* imm[4:0] + (imm[11:5] << 5) */
        self.imm1() + (self.imm2() << 5)
    }
}
