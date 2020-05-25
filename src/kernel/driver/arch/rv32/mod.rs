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

pub struct RV32 {
    pub index: u32,
    pub modules: String,
    pub mtvec: Mtvec,
    pub mie: Mie,
    pub mip: Mip,
    pub mepc: Mepc,
    pub mstatus: Mstatus,
    pub mcause: Mcause,
}

impl RV32 {
    pub fn new(index: u32, modules: String) -> Self {
        RV32{index, modules, mtvec: Mtvec {}, mie: Mie {}, mip: Mip {}, mepc: Mepc {}, mstatus: Mstatus {}, mcause: Mcause {}, }
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

use crate::kernel::table::Table;
use crate::kernel::interface::vkth;

/// 割込みハンドラ
#[no_mangle]
pub extern "C" fn interrupt_handler() {
    unsafe {
        let res = &mut Table::table().resource;
        res.io.timer.disable_interrupt();
        res.cpu.disable_interrupt();
    }
    vkth::int::entry();
}

/// 割込みトラップのエントリポイント
#[cfg(all(target_arch = "riscv32", target_os = "none"))]
//#[link_section = ".riscv.trap"]
#[export_name = "_start_trap"]
#[naked]
pub extern "C" fn _start_trap() {
    unsafe {
        asm! ("
        // from kernel
        .align 4
            csrrw sp, 0x340, sp // CSR=0x340=mscratch

            addi sp, sp, -16*4

            csrw mepc, ra

            // Store registers
            sw   ra, 0*4(sp)
            sw   t0, 1*4(sp)
            sw   t1, 2*4(sp)
            sw   t2, 3*4(sp)
            sw   t3, 4*4(sp)
            sw   t4, 5*4(sp)
            sw   t5, 6*4(sp)
            sw   t6, 7*4(sp)
            sw   a0, 8*4(sp)
            sw   a1, 9*4(sp)
            sw   a2, 10*4(sp)
            sw   a3, 11*4(sp)
            sw   a4, 12*4(sp)
            sw   a5, 13*4(sp)
            sw   a6, 14*4(sp)
            sw   a7, 15*4(sp)

            jal ra, interrupt_handler

            // Restore the registers from the stack.
            lw   ra, 0*4(sp)
            lw   t0, 1*4(sp)
            lw   t1, 2*4(sp)
            lw   t2, 3*4(sp)
            lw   t3, 4*4(sp)
            lw   t4, 5*4(sp)
            lw   t5, 6*4(sp)
            lw   t6, 7*4(sp)
            lw   a0, 8*4(sp)
            lw   a1, 9*4(sp)
            lw   a2, 10*4(sp)
            lw   a3, 11*4(sp)
            lw   a4, 12*4(sp)
            lw   a5, 13*4(sp)
            lw   a6, 14*4(sp)
            lw   a7, 15*4(sp)

            addi sp, sp, 16*4

            mret
        "
        :
        :
        :
        : "volatile");
    }
}