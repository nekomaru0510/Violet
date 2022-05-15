//! mhartid csr

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub mhartid [
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    ]
}

#[derive(Clone)]
pub struct Mhartid;

impl RegisterReadWrite<u64, mhartid::Register> for Mhartid {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, mhartid" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        
    }
}

