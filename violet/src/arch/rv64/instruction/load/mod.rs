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
                /* Store命令のオペコードは共通 */
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
                /* Store命令のオペコードは共通 */
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

    /* rdレジスタのインデックスを取得 */
    pub fn dst(&self) -> usize {
        match self {
            Load::Lb(lb) => lb.rd(),
            Load::Lh(lh) => lh.rd(),
            Load::Lw(lw) => lw.rd(),
            Load::Ld(ld) => ld.rd(),
            Load::Clw(clw) => clw.rd() + 8, /* 圧縮命令は、s0-a5までしか表現できない */
            Load::Cld(cld) => cld.rd() + 8, /* 圧縮命令は、s0-a5までしか表現できない */
            Load::Clq(clq) => clq.rd() + 8, /* 圧縮命令は、s0-a5までしか表現できない */
            _ => 0,
        }
    }
}

#[cfg(test)]
use crate::driver::arch::rv64::regs;
//use regs::Registers;
#[test_case]
fn test_load() -> Result<(), &'static str> {
    let inst = 0x618c; /* ld      a1,0(a1) */
    if Load::from_val(inst).dst() == regs::A1 {
        Ok(())
    } else {
        Err("Failed to fetch load register")
    }
}
