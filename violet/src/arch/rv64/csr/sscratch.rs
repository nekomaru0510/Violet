//! Supervisor Scratch Register(sscratch)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub sscratch [
        SSCRATCH       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Sscratch;

impl RegisterReadWrite<u64, sscratch::Register> for Sscratch {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, sscratch" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw sscratch, $0" :: "r"(value) :: "volatile");
        }
    }
}
