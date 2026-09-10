//! Hypervisor timer delta register(htimedelta)

use crate::register;

register!(
    Htimedelta,         /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x605",   /* Read Instruction */
    "csrw 0x605, {}",   /* Write Instruction */
    {                   /* Register Field */
        HTIMEDELTA      OFFSET(0)  NUMBITS(64) []
    }
);
