//! Hypervisor Status Register (hstatus)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hstatus [
        /// S-mode Interrupt Enable
        VSBE       OFFSET(5)  NUMBITS(1) [],
        GVA        OFFSET(6)  NUMBITS(1) [],
        SPV        OFFSET(7)  NUMBITS(1) [],
        SPVP       OFFSET(8)  NUMBITS(1) [],
        HU         OFFSET(9)  NUMBITS(1) [],
        VGEIN      OFFSET(12)  NUMBITS(6) [],
        VTVM       OFFSET(20)  NUMBITS(1) [],
        VTW        OFFSET(21)  NUMBITS(1) [],
        VTSR       OFFSET(22)  NUMBITS(1) [],
        VSXL       OFFSET(32)  NUMBITS(2) []
    ]
}

#[derive(Clone)]
pub struct Hstatus;

impl RegisterReadWrite<u64, hstatus::Register> for Hstatus {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u64 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x600" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u64) {
        unsafe {
            asm!("csrw 0x600, $0" :: "r"(value) :: "volatile");
        }
    }
}
