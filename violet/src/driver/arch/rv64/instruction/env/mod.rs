//! Environment Instructions

pub mod ebreak;
pub mod ecall;

use crate::driver::arch::rv64::instruction;
use instruction::format::lformat::LFormat;

use ebreak::Ebreak;
use ecall::Ecall;

pub trait EnvT {
    fn new(inst: usize) -> Self;
    fn imm(&self) -> usize;
}

pub enum Env {
    Ecall(Ecall),
    Ebreak(Ebreak),
    UNIMP,
}

impl Env {
    pub fn from_val(inst: usize) -> Self {
        match (LFormat { inst }.imm()) {
            Ecall::IMM => Env::Ecall(Ecall::new(inst)),
            Ebreak::IMM => Env::Ebreak(Ebreak::new(inst)),
            _ => Env::UNIMP,
        }
    }
}

#[test_case]
fn test_env() -> Result<(), &'static str> {
    let inst = 0x00000073; /* ecall */
    match Env::from_val(inst) {
        Env::Ecall(e) => Ok(()),
        Env::Ebreak(e) => Err("Failed to identify ecall instruction"),
        _ => Err("Failed to identify ecall instruction"),
    }
}
