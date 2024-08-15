//! Supervisor Cause Register(scause)

use crate::register;

register!(
    Scause,             /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x142",   /* Read Instruction */
    "csrw 0x142, {}",   /* Write Instruction */
    {                   /* Register Field */
        EXCEPTION       OFFSET(0)  NUMBITS(63) [],
        INTERRUPT       OFFSET(63)  NUMBITS(1) []
    }
);
