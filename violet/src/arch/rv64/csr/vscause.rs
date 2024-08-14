//! Virtual Supervisor Cause Register(vscause)

use crate::register;

register!(
    Vscause,            /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x242",   /* Read Instruction */
    "csrw 0x242, $0",   /* Write Instruction */
    {                   /* Register Field */
        EXCEPTION       OFFSET(0)  NUMBITS(63) [],
        INTERRUPT       OFFSET(63)  NUMBITS(1) []
    }
);
