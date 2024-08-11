//! cld Instruction

use super::LoadT;
use crate::arch::rv64::instruction;
use instruction::format::clformat::CLFormat;

pub struct Cld {
    inst: CLFormat,
}

impl LoadT for Cld {
    fn new(inst: usize) -> Self {
        Cld {
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
        /* imm[5:3] + imm[7:6] */
        (self.inst.imm2() << 3) + (self.inst.imm1() << 6)
    }
}

impl Cld {
    pub const FUNCT3: usize = 0b011;
    pub const OPCODE: usize = 0b00;
}
