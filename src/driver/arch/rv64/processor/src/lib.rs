#![feature(llvm_asm)]


#![no_std]

extern crate register;
use register::{cpu::RegisterReadWrite/*, register_bitfields*/};

extern crate alloc;
use alloc::string::String;

pub mod csr;
use csr::mtvec::*;
use csr::mie::*;
use csr::mip::*;
use csr::mepc::*;
use csr::mstatus::*;
use csr::mcause::*;

pub struct Processor {
    pub index: u32,
    pub modules: String,
    pub mtvec: Mtvec,
    pub mie: Mie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub mcause: Mcause,
}

impl Processor {
    pub fn new(index: u32, modules: String) -> Self {
        Processor{index, modules, mtvec: Mtvec {}, mie: Mie {}, mip: Mip {}, mepc: Mepc {}, mstatus: Mstatus {}, mcause: Mcause {}, }
    }

    pub fn enable_interrupt(&self) {
        self.mie.modify(mie::MSIE::SET);
        self.mie.modify(mie::MTIE::SET);
        self.mie.modify(mie::MEIE::SET);
        self.mstatus.modify(mstatus::MIE::SET);
    }

    pub fn disable_interrupt(&self) {
        self.mie.modify(mie::MSIE::CLEAR);
        self.mie.modify(mie::MTIE::CLEAR);
        self.mie.modify(mie::MEIE::CLEAR);
        self.mstatus.modify(mstatus::MIE::CLEAR);
    }

}
