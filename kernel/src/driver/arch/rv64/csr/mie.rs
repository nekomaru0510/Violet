//! Machine interrupt-enable register(mie)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub mie [
        /// Software Interrpt Enable
        USIE      OFFSET(0)  NUMBITS(1) [],
        SSIE      OFFSET(1)  NUMBITS(1) [],
        //WPRI      OFFSET(2)  NUMBITS(1) [],
        MSIE      OFFSET(3)  NUMBITS(1) [],

        /// Timer Interrupt Enable
        UTIE      OFFSET(4)  NUMBITS(1) [],
        STIE      OFFSET(5)  NUMBITS(1) [],
        //WPRI      OFFSET(6)  NUMBITS(1) [],
        MTIE      OFFSET(7)  NUMBITS(1) [],

        /// External Interrupt Enable
        UEIE      OFFSET(8)  NUMBITS(1) [],
        SEIE      OFFSET(9)  NUMBITS(1) [],
        //WPRI      OFFSET(10) NUMBITS(1) [],
        MEIE      OFFSET(11) NUMBITS(1) []
        //WPRE      OFFSET(12) NUMBITS(20) []
    ]
}

#[derive(Clone)]
pub struct Mie;

impl RegisterReadWrite<u32, mie::Register> for Mie {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            asm!("csrr $0, mie" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            asm!("csrw mie, $0" :: "r"(value) :: "volatile");
        }
    }
}