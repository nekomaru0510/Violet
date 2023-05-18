//! B Format

use crate::{bitfield, bit_extract, bit_fill};

pub struct BFormat {
    pub inst: usize,
}

impl BFormat {
    bitfield!(OPCODE:[6,0]);
    bitfield!(IMM1:[7,7]);      /* imm[11] */
    bitfield!(IMM2:[11,8]);     /* imm[4:1] */
    bitfield!(FUNCT3:[14,12]);
    bitfield!(RS1:[19,15]);
    bitfield!(RS2:[24,20]);
    bitfield!(IMM3:[30,25]);     /* imm[10:5] */
    bitfield!(IMM4:[31,31]);     /* imm[12] */

    pub fn opcode(&self) -> usize {
        bit_extract!(self.inst, Self::OPCODE)
    }

    pub fn imm1(&self) -> usize {
        bit_extract!(self.inst, Self::IMM1)
    }

    pub fn imm2(&self) -> usize {
        bit_extract!(self.inst, Self::IMM2)
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

    pub fn imm3(&self) -> usize {
        bit_extract!(self.inst, Self::IMM3)
    }
    
    pub fn imm4(&self) -> usize {
        bit_extract!(self.inst, Self::IMM4)
    }

}
