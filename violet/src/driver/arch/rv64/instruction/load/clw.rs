//! clw Instruction

use super::LoadT;
use crate::driver::arch::rv64::instruction;
use instruction::format::clformat::CLFormat;

pub struct Clw {
    inst: CLFormat,
}

impl LoadT for Clw {
    fn new(inst: usize) -> Self {
        Clw {
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
        /* imm[5:3] + imm[2|6] */
        let imm1 = self.inst.imm1();
        let bit6 = imm1 & 0b01;
        let bit2 = imm1 & 0b10;
        (bit6 << 6) + (self.inst.imm2() << 3) + (bit2 << 2)
    }
}

impl Clw {
    pub const FUNCT3: usize = 0b010;
    pub const OPCODE: usize = 0b00;
}
