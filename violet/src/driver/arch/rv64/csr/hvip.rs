//! Hypervisor virtual interrupt pending Register (hvip)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hvip [
        VSSIP     OFFSET(2)  NUMBITS(1) [],
        VSTIP     OFFSET(6)  NUMBITS(1) [],
        VSEIP     OFFSET(10) NUMBITS(1) []
    ]
}

#[derive(Clone)]
pub struct Hvip;

impl RegisterReadWrite<u64, hvip::Register> for Hvip {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x645" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x645, $0" :: "r"(value) :: "volatile");
        }
    }
}
