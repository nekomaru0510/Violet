//! csrrci Instruction

use super::CsrT;
use crate::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

pub struct Csrrci {
    inst: LFormat,
}

impl CsrT for Csrrci {
    fn new(inst: usize) -> Self {
        Csrrci {
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

impl Csrrci {
    pub const FUNCT3: usize = 0b111;
    pub const OPCODE: usize = 0b1110011;
}
