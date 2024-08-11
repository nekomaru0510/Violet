//! Store Instructions

pub mod csd;
pub mod csq;
pub mod csw;
pub mod sb;
pub mod sd;
pub mod sh;
pub mod sw;

use crate::arch::rv64::instruction;
use crate::arch::rv64::regs;
use regs::Registers;

use instruction::format::csformat::CSFormat;
use instruction::format::sformat::SFormat;

use csd::Csd;
use csq::Csq;
use csw::Csw;
use sb::Sb;
use sd::Sd;
use sh::Sh;
use sw::Sw;

use instruction::Instruction;

pub trait StoreT {
    fn new(inst: usize) -> Self;
    fn rs1(&self) -> usize;
    fn rs2(&self) -> usize;
    fn offset(&self) -> usize;
}

pub enum Store {
    Sb(Sb),
    Sh(Sh),
    Sw(Sw),
    Sd(Sd),
    Csw(Csw),
    Csd(Csd),
    Csq(Csq),
    UNIMP,
}

impl Store {
    pub fn from_val(inst: usize) -> Self {
        if Instruction::is_compressed(inst) {
            if (CSFormat { inst }.op() == Csw::OPCODE) {
                /* Store命令のオペコードは共通 */
                match (CSFormat { inst }.funct3()) {
                    Csw::FUNCT3 => Store::Csw(Csw::new(inst)),
                    Csd::FUNCT3 => Store::Csd(Csd::new(inst)),
                    Csq::FUNCT3 => Store::Csq(Csq::new(inst)),
                    _ => Store::UNIMP,
                }
            } else {
                Store::UNIMP
            }
        } else {
            if (SFormat { inst }.opcode() == Sb::OPCODE) {
                /* Store命令のオペコードは共通 */
                match (SFormat { inst }.funct3()) {
                    Sb::FUNCT3 => Store::Sb(Sb::new(inst)),
                    Sh::FUNCT3 => Store::Sh(Sh::new(inst)),
                    Sw::FUNCT3 => Store::Sw(Sw::new(inst)),
                    Sd::FUNCT3 => Store::Sd(Sd::new(inst)),
                    _ => Store::UNIMP,
                }
            } else {
                Store::UNIMP
            }
        }
    }

    /* ソースレジスタのインデックスを取得 */
    pub fn src(&self) -> usize {
        match self {
            Store::Sb(sb) => sb.rs2(),
            Store::Sh(sh) => sh.rs2(),
            Store::Sw(sw) => sw.rs2(),
            Store::Sd(sd) => sd.rs2(),
            Store::Csw(csw) => csw.rs2() + 8, /* 圧縮命令は、s0-a5までしか表現できない */
            Store::Csd(csd) => csd.rs2() + 8, /* 圧縮命令は、s0-a5までしか表現できない */
            Store::Csq(csq) => csq.rs2() + 8, /* 圧縮命令は、s0-a5までしか表現できない */
            _ => 0,
        }
    }

    pub fn store_value(&self, regs: &Registers) -> usize {
        regs.reg[self.src()]
    }
}

#[test_case]
fn test_store() -> Result<(), &'static str> {
    let inst = 0xfcf43423; /* sd      a5,-56(s0) */
    if Store::from_val(inst).src() == regs::A5 {
        Ok(())
    } else {
        Err("Failed to fetch store register")
    }
}
