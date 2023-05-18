//! sb Instruction

use super::StoreT;
use crate::driver::arch::rv64::instruction;
use instruction::format::sformat::SFormat;

pub struct Sb {
    inst: SFormat,
}

impl StoreT for Sb {
    fn new(inst: usize) -> Self {
        Sb {
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

impl Sb {
    pub const FUNCT3: usize = 0b000;
    pub const OPCODE: usize = 0b0100011;
}
