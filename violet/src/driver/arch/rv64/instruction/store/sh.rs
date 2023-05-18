//! sh Instruction

use super::StoreT;
use crate::driver::arch::rv64::instruction;
use instruction::format::sformat::SFormat;

pub struct Sh {
    inst: SFormat,
}

impl StoreT for Sh {
    fn new(inst: usize) -> Self {
        Sh {
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

impl Sh {
    pub const FUNCT3: usize = 0b001;
    pub const OPCODE: usize = 0b0100011;
}
