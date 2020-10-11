//! Machine Status Register (mstatus)

extern crate register;
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub mstatus [
        /// U-mode InterruptEnable
        UIE       OFFSET(0)  NUMBITS(1) [], 
        /// S-mode Interrupt Enable
        SIE       OFFSET(1)  NUMBITS(1) [],
        //WPRI      OFFSET(2)  NUMBITS(1) [],
        /// M-mode Interrupt Enable
        MIE       OFFSET(3)  NUMBITS(1) [],

        UPIE      OFFSET(4)  NUMBITS(1) [],
        SPIE      OFFSET(5)  NUMBITS(1) [],
        //WPRI      OFFSET(6)  NUMBITS(1) [],
        MPIE      OFFSET(7)  NUMBITS(1) [],

        SPP       OFFSET(8)  NUMBITS(1) [],
        //WPRI      OFFSET(9)  NUMBITS(2) [],
        MPP       OFFSET(11) NUMBITS(2) [
            USER = 0,
            SUPERVISOR = 1,
            RESERVED = 2,
            MACHINE = 3
        ],
        FS        OFFSET(13) NUMBITS(2) [],
        XS        OFFSET(15) NUMBITS(2) [],
        MPRV      OFFSET(17) NUMBITS(1) [],
        SUM       OFFSET(18) NUMBITS(1) [],
        MXR       OFFSET(19) NUMBITS(1) [],
        TVM       OFFSET(20) NUMBITS(1) [],
        TW        OFFSET(21) NUMBITS(1) [],
        TSR       OFFSET(22) NUMBITS(1) [],
        WPRI      OFFSET(23) NUMBITS(8) [],
        SD        OFFSET(31) NUMBITS(1) []
    ]
}

pub struct Mstatus;

impl RegisterReadWrite<u32, mstatus::Register> for Mstatus {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            llvm_asm!("csrr $0, mstatus" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            llvm_asm!("csrw mstatus, $0" :: "r"(value) :: "volatile");
        }
    }
}
