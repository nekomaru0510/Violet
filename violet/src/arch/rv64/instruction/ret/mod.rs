//! Return Instructions

pub mod mret;
pub mod sret;

use crate::arch::rv64::instruction;
use instruction::format::rformat::RFormat;

use mret::Mret;
use sret::Sret;

pub trait RetT {
    fn new(inst: usize) -> Self;
    fn rs2(&self) -> usize;
    fn funct7(&self) -> usize;
}

pub enum Ret {
    Sret(Sret),
    Mret(Mret),
    UNIMP,
}

impl Ret {
    pub fn from_val(inst: usize) -> Self {
        match (RFormat { inst }.funct7()) {
            Sret::FUNCT7 => Ret::Sret(Sret::new(inst)),
            Mret::FUNCT7 => Ret::Mret(Mret::new(inst)),
            _ => Ret::UNIMP,
        }
    }
}

#[test_case]
fn test_ret() -> Result<(), &'static str> {
    let inst = 0x10200073; /* sret */
    match Ret::from_val(inst) {
        Ret::Sret(r) => Ok(()),
        Ret::Mret(r) => Err("Failed to identify sret instruction"),
        _ => Err("Failed to identify sret instruction"),
    }
}
