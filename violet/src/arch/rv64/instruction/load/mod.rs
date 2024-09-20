//! Load Instructions

pub mod lb;
pub mod ld;
pub mod lh;
pub mod lw;
//pub mod lbu;
//pub mod lhu;
pub mod cld;
pub mod clq;
pub mod clw;

use crate::arch::rv64::instruction;

use instruction::format::clformat::CLFormat;
use instruction::format::lformat::LFormat;

use cld::Cld;
use clq::Clq;
use clw::Clw;
use lb::Lb;
use ld::Ld;
use lh::Lh;
use lw::Lw;

use instruction::Instruction;

pub trait LoadT {
    fn new(inst: usize) -> Self;
    fn rd(&self) -> usize;
    fn rs1(&self) -> usize;
    fn offset(&self) -> usize;
}

pub enum Load {
    Lb(Lb),
    Lh(Lh),
    Lw(Lw),
    Ld(Ld),
    Clw(Clw),
    Cld(Cld),
    Clq(Clq),
    UNIMP,
}

impl Load {
    pub fn from_val(inst: usize) -> Self {
        if Instruction::is_compressed(inst) {
            if (CLFormat { inst }.op() == Clw::OPCODE) {
                // The opcode of the Load instruction is common
                match (CLFormat { inst }.funct3()) {
                    Clw::FUNCT3 => Load::Clw(Clw::new(inst)),
                    Cld::FUNCT3 => Load::Cld(Cld::new(inst)),
                    Clq::FUNCT3 => Load::Clq(Clq::new(inst)),
                    _ => Load::UNIMP,
                }
            } else {
                Load::UNIMP
            }
        } else {
            if (LFormat { inst }.opcode() == Lb::OPCODE) {
                // The opcode of the Load instruction is common
                match (LFormat { inst }.funct3()) {
                    Lb::FUNCT3 => Load::Lb(Lb::new(inst)),
                    Lh::FUNCT3 => Load::Lh(Lh::new(inst)),
                    Lw::FUNCT3 => Load::Lw(Lw::new(inst)),
                    Ld::FUNCT3 => Load::Ld(Ld::new(inst)),
                    _ => Load::UNIMP,
                }
            } else {
                Load::UNIMP
            }
        }
    }

    // Get the index of the rd register
    pub fn dst(&self) -> usize {
        match self {
            Load::Lb(lb) => lb.rd(),
            Load::Lh(lh) => lh.rd(),
            Load::Lw(lw) => lw.rd(),
            Load::Ld(ld) => ld.rd(),
            Load::Clw(clw) => clw.rd() + 8, // Compressed instructions can only express s0-a5
            Load::Cld(cld) => cld.rd() + 8,
            Load::Clq(clq) => clq.rd() + 8,
            _ => 0,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Load::Lb(_) => 1,
            Load::Lh(_) => 2,
            Load::Lw(_) => 4,
            Load::Ld(_) => 8,
            Load::Clw(_) => 4,
            Load::Cld(_) => 8,
            Load::Clq(_) => 16,
            _ => 0,
        }
    }
}

#[cfg(test)]
use crate::arch::rv64::regs;
#[test_case]
fn test_load() -> Result<(), &'static str> {
    let inst = 0x618c; /* ld      a1,0(a1) */
    if Load::from_val(inst).dst() == regs::A1 {
        Ok(())
    } else {
        Err("Failed to fetch load register")
    }
}
