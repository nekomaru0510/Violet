//! CSR access Instructions

pub mod csrnumber;
pub mod csrrc;
pub mod csrrci;
pub mod csrrs;
pub mod csrrsi;
pub mod csrrw;
pub mod csrrwi;

use crate::arch::rv64::instruction;
use crate::arch::rv64::regs;
use instruction::format::lformat::LFormat;
use regs::Registers;

use csrrc::Csrrc;
use csrrci::Csrrci;
use csrrs::Csrrs;
use csrrsi::Csrrsi;
use csrrw::Csrrw;
use csrrwi::Csrrwi;

pub trait CsrT {
    fn new(inst: usize) -> Self;
    fn rd(&self) -> usize;
    fn rs1(&self) -> usize;
    fn zimm(&self) -> usize;
    fn csr(&self) -> usize;
}

pub enum Csr {
    Csrrw(Csrrw),
    Csrrs(Csrrs),
    Csrrc(Csrrc),
    Csrrwi(Csrrwi),
    Csrrsi(Csrrsi),
    Csrrci(Csrrci),
    UNIMP,
}

impl Csr {
    pub fn from_val(inst: usize) -> Self {
        if (LFormat { inst }.opcode() == Csrrw::OPCODE) {
            // The opcode of the CSR instruction is common
            match (LFormat { inst }.funct3()) {
                Csrrw::FUNCT3 => Csr::Csrrw(Csrrw::new(inst)),
                Csrrs::FUNCT3 => Csr::Csrrs(Csrrs::new(inst)),
                Csrrc::FUNCT3 => Csr::Csrrc(Csrrc::new(inst)),
                Csrrwi::FUNCT3 => Csr::Csrrwi(Csrrwi::new(inst)),
                Csrrsi::FUNCT3 => Csr::Csrrsi(Csrrsi::new(inst)),
                Csrrci::FUNCT3 => Csr::Csrrci(Csrrci::new(inst)),
                _ => Csr::UNIMP,
            }
        } else {
            Csr::UNIMP
        }
    }

    // Get the CSR number when accessing CSR
    pub fn csr(&self) -> usize {
        match self {
            Csr::Csrrw(csrrw) => csrrw.csr(),
            Csr::Csrrs(csrrs) => csrrs.csr(),
            Csr::Csrrc(csrrc) => csrrc.csr(),
            Csr::Csrrwi(csrrwi) => csrrwi.csr(),
            Csr::Csrrsi(csrrsi) => csrrsi.csr(),
            Csr::Csrrci(csrrci) => csrrci.csr(),
            _ => 0,
        }
    }

    // Get the index of rd when accessing CSR
    pub fn dst(&self) -> usize {
        match self {
            Csr::Csrrw(csrrw) => csrrw.rd(),
            Csr::Csrrs(csrrs) => csrrs.rd(),
            Csr::Csrrc(csrrc) => csrrc.rd(),
            Csr::Csrrwi(csrrwi) => csrrwi.rd(),
            Csr::Csrrsi(csrrsi) => csrrsi.rd(),
            Csr::Csrrci(csrrci) => csrrci.rd(),
            _ => 0,
        }
    }

    // Get the index of rs1 when accessing CSR
    pub fn src(&self) -> usize {
        match self {
            Csr::Csrrw(csrrw) => csrrw.rs1(),
            Csr::Csrrs(csrrs) => csrrs.rs1(),
            Csr::Csrrc(csrrc) => csrrc.rs1(),
            Csr::Csrrwi(csrrwi) => csrrwi.rs1(),
            Csr::Csrrsi(csrrsi) => csrrsi.rs1(),
            Csr::Csrrci(csrrci) => csrrci.rs1(),
            _ => 0,
        }
    }

    // Get the immediate value used in the operation
    pub fn imm(&self, regs: &mut Registers) -> usize {
        match self {
            Csr::Csrrw(csrrw) => regs.reg[csrrw.rs1()],
            Csr::Csrrs(csrrs) => regs.reg[csrrs.rs1()],
            Csr::Csrrc(csrrc) => regs.reg[csrrc.rs1()],
            Csr::Csrrwi(csrrwi) => csrrwi.zimm(),
            Csr::Csrrsi(csrrsi) => csrrsi.zimm(),
            Csr::Csrrci(csrrci) => csrrci.zimm(),
            _ => 0,
        }
    }

    // Get the value to write to CSR
    pub fn write_val(&self, csr: usize, imm:usize) -> usize {
        match self {
            Csr::Csrrw(csrrw) => imm,
            Csr::Csrrs(csrrs) => csr | imm,
            Csr::Csrrc(csrrc) => csr & !imm,
            Csr::Csrrwi(csrrwi) => imm,
            Csr::Csrrsi(csrrsi) => csr | imm,
            Csr::Csrrci(csrrci) => csr & !imm,
            _ => 0,
        }
    }
}

#[test_case]
fn test_csr() -> Result<(), &'static str> {
    let inst = 0x10561073; /* csrw    stvec,a2 */
    /* csrrw x0, #0x105, a2 */
    if Csr::from_val(inst).src() == regs::A2 {
        if Csr::from_val(inst).csr() == 0x105 {
            Ok(())
        } else {
            Err("Failed to fetch csr number")
        }
    } else {
        Err("Failed to fetch CSR src register")
    }
}
