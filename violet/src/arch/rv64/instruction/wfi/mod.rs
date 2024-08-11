//! wfi Instruction

use crate::arch::rv64::instruction;
use instruction::format::rformat::RFormat;

pub struct Wfi {
    inst: RFormat,
}

impl Wfi {
    fn new(inst: usize) -> Self {
        Wfi {
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

impl Wfi {
    pub const FUNCT7: usize = 0b000_1000;
    pub const RS2: usize = 0b00101;
    pub const FUNCT3: usize = 0b000;
    pub const OPCODE: usize = 0b1110011;
}
