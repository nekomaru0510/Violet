//! csq Instruction

use super::StoreT;
use crate::arch::rv64::instruction;
use instruction::format::csformat::CSFormat;

pub struct Csq {
    inst: CSFormat,
}

impl StoreT for Csq {
    fn new(inst: usize) -> Self {
        Csq {
            inst: CSFormat { inst },
        }
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn rs2(&self) -> usize {
        self.inst.rs2()
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

impl Csq {
    pub const FUNCT3: usize = 0b101;
    pub const OPCODE: usize = 0b00;
}
