//! Hypervisor Interrupt Delegation Register (hideleg)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hideleg [
        HIDELEG       OFFSET(0)  NUMBITS(64) []
    ]
}

pub struct Hideleg;

impl RegisterReadWrite<u64, hideleg::Register> for Hideleg {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x603" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x603, $0" :: "r"(value) :: "volatile");
        }
    }
}
