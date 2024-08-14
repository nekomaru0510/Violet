//! Supervisor Exception Program Counter(sepc)

use crate::register;

register!(
    Sepc,               /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x141",   /* Read Instruction */
    "csrw 0x141, $0",   /* Write Instruction */
    {                   /* Register Field */
        SEPC       OFFSET(0)  NUMBITS(64) []
    }
);
