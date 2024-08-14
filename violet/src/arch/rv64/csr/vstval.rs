//! Virtual Supervisor Trap Value Register(vstval)

use crate::register;

register!(
    Vstval,             /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x243",   /* Read Instruction */
    "csrw 0x243, $0",   /* Write Instruction */
    {                   /* Register Field */
        VSTVAL       OFFSET(0)  NUMBITS(64) []
    }
);
