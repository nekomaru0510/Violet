//! Hypervisor Interrupt Delegation Register (hideleg)

use crate::register;

register!(
    Hideleg,            /* Register Name */
    u64,                /* Register Size */
    "csrr {}, 0x603",   /* Read Instruction */
    "csrw 0x603, {}",   /* Write Instruction */
    {                   /* Register Field */
        HIDELEG       OFFSET(0)  NUMBITS(64) []
    }
);
