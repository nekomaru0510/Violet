//! Hypervisor Exception Delegation Register (hedeleg)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hedeleg [
        HEDELEG       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Hedeleg;

impl RegisterReadWrite<u64, hedeleg::Register> for Hedeleg {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x602" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x602, $0" :: "r"(value) :: "volatile");
        }
    }
}
