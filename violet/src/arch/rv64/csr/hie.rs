//! Hypervisor interrupt-enable register(hie)

use crate::register;

register!(
    Hie,                /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x604",   /* Read Instruction */
    "csrw 0x604, {}",   /* Write Instruction */
    {                   /* Register Field */
        VSSIE      OFFSET(2)  NUMBITS(1) [],
        VSTIE      OFFSET(6)  NUMBITS(1) [],
        VSEIE      OFFSET(10) NUMBITS(1) [],
        SGEIE      OFFSET(12) NUMBITS(1) []
    }
);
