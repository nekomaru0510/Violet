//! Hypervisor guest external interrupt-enable register (hgeie)

use crate::register;

register!(
    Hgeie,          /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x607",   /* Read Instruction */
    "csrw 0x607, {}",   /* Write Instruction */
    {                   /* Register Field */
        HGEIE       OFFSET(0)  NUMBITS(64) []
    }
);
