//! clq Instruction

use super::LoadT;
use crate::driver::arch::rv64::instruction;
use instruction::format::clformat::CLFormat;

pub struct Clq {
    inst: CLFormat,
}

impl LoadT for Clq {
    fn new(inst: usize) -> Self {
        Clq {
            inst: CLFormat { inst },
        }
    }

    fn rd(&self) -> usize {
        self.inst.rd()
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn offset(&self) -> usize {
        /* imm[5|4|8] + imm[7:6] */
        let imm2 = self.inst.imm2();
        let bit5 = imm2 & 0b100;
        let bit4 = imm2 & 0b010;
        let bit8 = imm2 & 0b001;
        (bit5 << 5) + (bit4 << 4) + (bit8 << 8) + (self.inst.imm1() << 6)
    }
}

impl Clq {
    pub const FUNCT3: usize = 0b001;
    pub const OPCODE: usize = 0b00;
}
