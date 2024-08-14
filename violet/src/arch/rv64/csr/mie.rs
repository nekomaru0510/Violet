//! Machine interrupt-enable register(mie)

use crate::register;

register!(
    Mie,                /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x304",   /* Read Instruction */
    "csrw 0x304, $0",   /* Write Instruction */
    {                   /* Register Field */
        // Software Interrpt Enable
        USIE      OFFSET(0)  NUMBITS(1) [],
        SSIE      OFFSET(1)  NUMBITS(1) [],
        //WPRI      OFFSET(2)  NUMBITS(1) [],
        MSIE      OFFSET(3)  NUMBITS(1) [],

        // Timer Interrupt Enable
        UTIE      OFFSET(4)  NUMBITS(1) [],
        STIE      OFFSET(5)  NUMBITS(1) [],
        //WPRI      OFFSET(6)  NUMBITS(1) [],
        MTIE      OFFSET(7)  NUMBITS(1) [],

        // External Interrupt Enable
        UEIE      OFFSET(8)  NUMBITS(1) [],
        SEIE      OFFSET(9)  NUMBITS(1) [],
        //WPRI      OFFSET(10) NUMBITS(1) [],
        MEIE      OFFSET(11) NUMBITS(1) []
        //WPRE      OFFSET(12) NUMBITS(20) []
    }
);
