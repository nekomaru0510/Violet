//! sw Instruction

use super::StoreT;
use crate::arch::rv64::instruction;
use instruction::format::sformat::SFormat;

pub struct Sw {
    inst: SFormat,
}

impl StoreT for Sw {
    fn new(inst: usize) -> Self {
        Sw {
            inst: SFormat { inst },
        }
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn rs2(&self) -> usize {
        self.inst.rs2()
    }

    fn offset(&self) -> usize {
        self.inst.imm()
    }
}

impl Sw {
    pub const FUNCT3: usize = 0b010;
    pub const OPCODE: usize = 0b0100011;
}
