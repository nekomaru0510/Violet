//! ecall Instruction

use super::EnvT;
use crate::driver::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Ecall {
    inst: LFormat,
}

impl EnvT for Ecall {
    fn new(inst: usize) -> Self {
        Ecall {
            inst: LFormat { inst },
        }
    }

    fn imm(&self) -> usize {
        self.inst.imm()
    }
}

impl Ecall {
    pub const IMM: usize = 0b0000_0000_0000;
    pub const FUNCT3: usize = 0b000;
    pub const OPCODE: usize = 0b1110011;
}
