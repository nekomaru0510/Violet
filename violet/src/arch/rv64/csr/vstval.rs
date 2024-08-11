//! Virtual Supervisor Trap Value Register(vstval)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub vstval [
        VSTVAL       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Vstval;

impl RegisterReadWrite<u64, vstval::Register> for Vstval {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x243" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x243, $0" :: "r"(value) :: "volatile");
        }
    }
}
