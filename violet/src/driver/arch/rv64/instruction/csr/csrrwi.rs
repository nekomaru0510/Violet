//! csrrwi Instruction

use super::CsrT;
use crate::driver::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Csrrwi {
    inst: LFormat,
}

impl CsrT for Csrrwi {
    fn new(inst: usize) -> Self {
        Csrrwi {
            inst: LFormat { inst },
        }
    }

    fn rd(&self) -> usize {
        self.inst.rd()
    }

    fn rs1(&self) -> usize {
        self.inst.rs1()
    }

    fn zimm(&self) -> usize {
        self.inst.rs1()
    }

    fn csr(&self) -> usize {
        self.inst.imm()
    }
}

impl Csrrwi {
    pub const FUNCT3: usize = 0b101;
    pub const OPCODE: usize = 0b1110011;
}
