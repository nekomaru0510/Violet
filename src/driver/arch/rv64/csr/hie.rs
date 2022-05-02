//! Hypervisor interrupt-enable register(hie)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub hie [
        /// Software Interrpt Enable
        USIE      OFFSET(0)  NUMBITS(1) [],
        SSIE      OFFSET(1)  NUMBITS(1) [],
        VSSIE      OFFSET(2)  NUMBITS(1) [],
        MSIE      OFFSET(3)  NUMBITS(1) [],

        /// Timer Interrupt Enable
        UTIE      OFFSET(4)  NUMBITS(1) [],
        STIE      OFFSET(5)  NUMBITS(1) [],
        VSTIE      OFFSET(6)  NUMBITS(1) [],
        MTIE      OFFSET(7)  NUMBITS(1) [],

        /// External Interrupt Enable
        UEIE      OFFSET(8)  NUMBITS(1) [],
        SEIE      OFFSET(9)  NUMBITS(1) [],
        VSEIE      OFFSET(10) NUMBITS(1) [],
        MEIE      OFFSET(11) NUMBITS(1) []
        //WPRE      OFFSET(12) NUMBITS(20) []
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