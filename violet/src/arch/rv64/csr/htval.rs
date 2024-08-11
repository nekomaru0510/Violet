//! Hypervisor Trap Value Register(htval)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub htval [
        HTVAL       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Htval;

impl RegisterReadWrite<u64, htval::Register> for Htval {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x643" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x643, $0" :: "r"(value) :: "volatile");
        }
    }
}
