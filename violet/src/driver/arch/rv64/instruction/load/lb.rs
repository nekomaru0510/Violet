//! lb Instruction

use super::LoadT;
use crate::driver::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Lb {
    inst: LFormat,
}

impl LoadT for Lb {
    fn new(inst: usize) -> Self {
        Lb {
            inst: LFormat { inst },
        }
    }

    fn rd(&self) -> usize {
        self.inst.rd()
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn offset(&self) -> usize {
        self.inst.imm()
    }
}

impl Lb {
    pub const FUNCT3: usize = 0b000;
    pub const OPCODE: usize = 0b0000011;
}
