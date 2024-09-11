//! Supervisor Status Register (sstatus)

use crate::register;

register!(
    Sstatus,            /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x100",   /* Read Instruction */
    "csrw 0x100, {}",   /* Write Instruction */
    {                   /* Register Field */
        // U-mode InterruptEnable
        UIE       OFFSET(0)  NUMBITS(1) [],
        // S-mode Interrupt Enable
        SIE       OFFSET(1)  NUMBITS(1) [],
        //WPRI      OFFSET(2)  NUMBITS(1) [],
        // M-mode Interrupt Enable
        MIE       OFFSET(3)  NUMBITS(1) [],

        // xPIE ... x-mode is Previous mode that is trapped interrupt
        UPIE      OFFSET(4)  NUMBITS(1) [],
        SPIE      OFFSET(5)  NUMBITS(1) [],
        //WPRI      OFFSET(6)  NUMBITS(1) [],
        MPIE      OFFSET(7)  NUMBITS(1) [],

        // xPP ... Original privilege mode that is trapped interrupt
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
    }
);
