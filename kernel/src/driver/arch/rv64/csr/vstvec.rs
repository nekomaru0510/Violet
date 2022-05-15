//! vstvec csr

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub vstvec [
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    ]
}

#[derive(Clone)]
pub struct Vstvec;

impl RegisterReadWrite<u64, vstvec::Register> for Vstvec {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x205" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x205, $0" :: "r"(value) :: "volatile");
        }
    }
}

