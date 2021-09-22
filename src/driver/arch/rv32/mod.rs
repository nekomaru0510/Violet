//! RV32I CPU ドライバ

#![feature(naked_functions)]

pub mod boot;

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
    pub mtvec: Mtvec,
    pub mie: Mie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub mcause: Mcause,
}

impl Processor {
    pub fn new(index: u32) -> Self {
        Processor{index, mtvec: Mtvec {}, mie: Mie {}, mip: Mip {}, mepc: Mepc {}, mstatus: Mstatus {}, mcause: Mcause {}, }
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

/* カーネル本体の割込みハンドラ */
use crate::interrupt_handler;
use crate::Context;

// CPU内 割込みハンドラ
#[no_mangle]
pub extern "C" fn get_context(sp :*mut usize) {

    let mut cont = Context::new();
    cont.regsize = 16;
    let ret = interrupt_handler(&mut cont);

    /* [todo fix] 割込みごとに(レジスタを読むために)毎回newするのはよろしくない気がするので、なるべくやめる */
    let cpu = Processor::new(0);
    cpu.mstatus.modify(mstatus::MPIE::SET); /* mstatusのMPIEには割込み元でのMIEビットが入る */
    cpu.mip.modify(mip::MTIP::CLEAR);       /* タイマ割込みがペンディングされてる？ためクリア(必要か？) */

}

