//! Virtual Supervisor Trap-Vector Base-Address (vstvec)

use crate::register;

register!(
    Vstvec,             /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x205",   /* Read Instruction */
    "csrw 0x205, $0",   /* Write Instruction */
    {                   /* Register Field */
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    }
);
