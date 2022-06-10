//! scause csr

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub scause [
        EXCEPTION       OFFSET(0)  NUMBITS(63) [],
        INTERRUPT       OFFSET(63)  NUMBITS(1) []
    ]
}

#[derive(Clone)]
pub struct Scause;

impl RegisterReadWrite<u64, scause::Register> for Scause {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, scause" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw scause, $0" :: "r"(value) :: "volatile");
        }
    }
}
