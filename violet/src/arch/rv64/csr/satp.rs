//! Supervisor address translation and protection Register (satp)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub satp [
        PPN       OFFSET(0)  NUMBITS(44) [],
        ASID      OFFSET(44)  NUMBITS(16) [],
        MODE      OFFSET(60)  NUMBITS(4) [
            BARE = 0,
            SV39X4 = 8,
            SV48X4 = 9,
            SV57X4 = 10 //Reserved
        ]
    ]
}

#[derive(Clone)]
pub struct Satp;

impl RegisterReadWrite<u64, satp::Register> for Satp {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, satp" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw satp, $0" :: "r"(value) :: "volatile");
        }
    }
}
