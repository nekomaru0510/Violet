//! stvec csr

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub stvec [
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    ]
}

pub struct Stvec;

impl RegisterReadWrite<u64, stvec::Register> for Stvec {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, stvec" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw stvec, $0" :: "r"(value) :: "volatile");
        }
    }
}

