//! Hypervisor interrupt-enable register(hie)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hie [
        /// Software Interrpt Enable
        VSSIE      OFFSET(2)  NUMBITS(1) [],

        /// Timer Interrupt Enable
        VSTIE      OFFSET(6)  NUMBITS(1) [],

        /// External Interrupt Enable
        VSEIE      OFFSET(10) NUMBITS(1) [],
        SGEIE      OFFSET(12) NUMBITS(1) []
    ]
}

#[derive(Clone)]
pub struct Hie;

impl RegisterReadWrite<u64, hie::Register> for Hie {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x604" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x604, $0" :: "r"(value) :: "volatile");
        }
    }
}
