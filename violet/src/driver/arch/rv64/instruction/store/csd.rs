//! csd Instruction

use super::StoreT;
use crate::driver::arch::rv64::instruction;
use instruction::format::csformat::CSFormat;

pub struct Csd {
    inst: CSFormat,
}

impl StoreT for Csd {
    fn new(inst: usize) -> Self {
        Csd {
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
        /* imm[5:3] + imm[7:6] */
        (self.inst.imm2() << 3) + (self.inst.imm1() << 6)
    }
}

impl Csd {
    pub const FUNCT3: usize = 0b111;
    pub const OPCODE: usize = 0b00;
}
