//! Time counter register (time)

use crate::register;

register!(
    Time,               /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0xc01",   /* Read Instruction */
    "csrw 0xc01, {}",   /* Write Instruction */
    {                   /* Register Field */
        TIME        OFFSET(0)  NUMBITS(64) []
    }
);