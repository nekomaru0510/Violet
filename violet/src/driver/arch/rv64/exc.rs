//! 例外モジュール

use super::csr::stval::*;

extern crate register;
use register::cpu::RegisterReadWrite;

#[derive(Clone)]
pub struct Rv64Exc {
    pub stval: Stval,
}

impl Rv64Exc {
    pub const fn new() -> Self {
        Rv64Exc { stval: Stval {} }
    }

    pub fn get_fault_address(&self) -> u64 {
        self.stval.get()
    }
}
