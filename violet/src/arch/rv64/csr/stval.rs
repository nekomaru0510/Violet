//! Supervisor Trap Value Register(stval)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub stval [
        STVAL       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Stval;

impl RegisterReadWrite<u64, stval::Register> for Stval {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, stval" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw stval, $0" :: "r"(value) :: "volatile");
        }
    }
}
