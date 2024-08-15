//! Hypervisor Trap Value Register(htval)

use crate::register;

register!(
    Htval,              /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x643",   /* Read Instruction */
    "csrw 0x643, {}",   /* Write Instruction */
    {                   /* Register Field */
        HTVAL       OFFSET(0)  NUMBITS(64) []
    }
);
