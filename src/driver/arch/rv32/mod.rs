//! RV32I CPU ドライバ

//#![feature(llvm_asm)]
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

/*

*/
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::uart::Uart;
use crate::driver::board::sifive_u::plic::Plic;
use crate::resource::io::serial::Serial;
use crate::library::std::Std;
use crate::print;
use crate::println;

/// 割込みハンドラ
#[no_mangle]
pub extern "C" fn interrupt_handler() {
    let uart = Uart::new(0x1001_0000);
    let serial = Serial::new(uart);
    let mut std = Std::new(serial);
    let intc = Plic::new(0x0C00_0000);
    println!(std, "int num : {}", intc.get_claim_complete());
    
    /*
    unsafe {
        let res = &mut Table::table().resource;
        res.io.timer.disable_interrupt();
        res.cpu.disable_interrupt();
    }
    vkth::int::entry();
    */
}

