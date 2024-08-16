//! Virtual Supervisor Status Register (vsstatus)

use crate::register;

register!(
    Vsstatus,           /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x200",   /* Read Instruction */
    "csrw 0x200, {}",   /* Write Instruction */
    {                   /* Register Field */
        // U-mode InterruptEnable
        UIE       OFFSET(0)  NUMBITS(1) [],
        // S-mode Interrupt Enable
        SIE       OFFSET(1)  NUMBITS(1) [],
        //WPRI      OFFSET(2)  NUMBITS(1) [],
        // M-mode Interrupt Enable
        MIE       OFFSET(3)  NUMBITS(1) [],

        // xPIE ... xは割込みがトラップされているモード
        UPIE      OFFSET(4)  NUMBITS(1) [],
        SPIE      OFFSET(5)  NUMBITS(1) [],
        //WPRI      OFFSET(6)  NUMBITS(1) [],
        MPIE      OFFSET(7)  NUMBITS(1) [],

        // xPP ... 割込み元の特権モード
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
        // Memory Privilege(1の際、MMUが有効化)
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