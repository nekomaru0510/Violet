//! mret Instruction

use super::RetT;
use crate::arch::rv64::instruction;
use instruction::format::rformat::RFormat;

pub struct Mret {
    inst: RFormat,
}

impl RetT for Mret {
    fn new(inst: usize) -> Self {
        Mret {
            inst: RFormat { inst },
        }
    }

    fn rs2(&self) -> usize {
        self.inst.rs2()
    }

    fn funct7(&self) -> usize {
        self.inst.funct7()
    }
}

impl Mret {
    pub const FUNCT7: usize = 0b001_1000;
    pub const RS2: usize = 0b00010;
    pub const FUNCT3: usize = 0b000;
    pub const OPCODE: usize = 0b1110011;
}
