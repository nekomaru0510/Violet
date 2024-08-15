//! Virtual Supervisor interrupt-pending register(vsip)

use crate::register;

register!(
    Vsip,               /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x244",   /* Read Instruction */
    "csrw 0x244, {}",   /* Write Instruction */
    {                   /* Register Field */
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
    }
);
