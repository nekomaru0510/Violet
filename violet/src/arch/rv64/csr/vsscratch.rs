//! Virtual Supervisor Scratch Register(vsscratch)

use crate::register;

register!(
    Vsscratch,          /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x240",   /* Read Instruction */
    "csrw 0x240, {}",   /* Write Instruction */
    {                   /* Register Field */
        VSSCRATCH       OFFSET(0)  NUMBITS(64) []
    }
);
