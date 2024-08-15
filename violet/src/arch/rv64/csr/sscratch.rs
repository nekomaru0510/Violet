//! Supervisor Scratch Register(sscratch)

use crate::register;

register!(
    Sscratch,           /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x140",   /* Read Instruction */
    "csrw 0x140, {}",   /* Write Instruction */
    {                   /* Register Field */
        SSCRATCH       OFFSET(0)  NUMBITS(64) []
    }
);
