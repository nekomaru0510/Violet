//! sd Instruction

use super::StoreT;
use crate::arch::rv64::instruction;
use instruction::format::sformat::SFormat;

pub struct Sd {
    inst: SFormat,
}

impl StoreT for Sd {
    fn new(inst: usize) -> Self {
        Sd {
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

impl Sd {
    pub const FUNCT3: usize = 0b011;
    pub const OPCODE: usize = 0b0100011;
}
