//! Supervisor Trap Value Register(stval)

use crate::register;

register!(
    Stval,              /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x143",   /* Read Instruction */
    "csrw 0x143, $0",   /* Write Instruction */
    {                   /* Register Field */
        STVAL       OFFSET(0)  NUMBITS(64) []
    }
);
