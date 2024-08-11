//! Hypervisor guest external interrupt-enable register (hgeie)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hgeie [
        HGEIE       OFFSET(0)  NUMBITS(64) []
    ]
}

#[derive(Clone)]
pub struct Hgeie;

impl RegisterReadWrite<u64, hgeie::Register> for Hgeie {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x607" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x607, $0" :: "r"(value) :: "volatile");
        }
    }
}
