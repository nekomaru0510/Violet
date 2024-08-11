//! csw Instruction

use super::StoreT;
use crate::arch::rv64::instruction;
use instruction::format::csformat::CSFormat;

pub struct Csw {
    inst: CSFormat,
}

impl StoreT for Csw {
    fn new(inst: usize) -> Self {
        Csw {
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
        /* imm[5:3] + imm[2|6] */
        let imm1 = self.inst.imm1();
        let bit6 = imm1 & 0b01;
        let bit2 = imm1 & 0b10;
        (bit6 << 6) + (self.inst.imm2() << 3) + (bit2 << 2)
    }
}

impl Csw {
    pub const FUNCT3: usize = 0b110;
    pub const OPCODE: usize = 0b00;
}
