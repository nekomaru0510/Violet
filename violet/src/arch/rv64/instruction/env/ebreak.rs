//! ebreak Instruction

use super::EnvT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Ebreak {
    inst: LFormat,
}

impl EnvT for Ebreak {
    fn new(inst: usize) -> Self {
        Ebreak {
            inst: LFormat { inst },
        }
    }

    fn imm(&self) -> usize {
        self.inst.imm()
    }
}

impl Ebreak {
    pub const IMM: usize = 0b0000_0000_0001;
    pub const FUNCT3: usize = 0b000;
    pub const OPCODE: usize = 0b1110011;
}
