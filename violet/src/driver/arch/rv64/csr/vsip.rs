//! Virtual Supervisor interrupt-pending register(vsip)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub vsip [
        USIP      OFFSET(0)  NUMBITS(1) [],
        SSIP      OFFSET(1)  NUMBITS(1) [],
        VSSIP     OFFSET(2)  NUMBITS(1) [],
        MSIP      OFFSET(3)  NUMBITS(1) [],

        UTIP      OFFSET(4)  NUMBITS(1) [],
        STIP      OFFSET(5)  NUMBITS(1) [],
        VSTIP     OFFSET(6)  NUMBITS(1) [],
        MTIP      OFFSET(7)  NUMBITS(1) [],

        UEIP      OFFSET(8)  NUMBITS(1) [],
        SEIP      OFFSET(9)  NUMBITS(1) [],
        VSEIP     OFFSET(10) NUMBITS(1) [],
        MEIP      OFFSET(11) NUMBITS(1) []
        //WPRI      OFFSET(12) NUMBITS(20) []
    ]
}

#[derive(Clone)]
pub struct Vsip;

impl RegisterReadWrite<u32, vsip::Register> for Vsip {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            asm!("csrr $0, 0x244" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            asm!("csrw 0x244, $0" :: "r"(value) :: "volatile");
        }
    }
}
