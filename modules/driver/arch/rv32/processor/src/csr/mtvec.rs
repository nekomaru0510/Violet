//! mtvec csr

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub mtvec [
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(30) []
    ]
}

pub struct Mtvec;

impl RegisterReadWrite<u32, mtvec::Register> for Mtvec {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            llvm_asm!("csrr $0, mtvec" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            llvm_asm!("csrw mtvec, $0" :: "r"(value) :: "volatile");
        }
    }
}

