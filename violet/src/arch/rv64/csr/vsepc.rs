//! Virtual Supervisor Exception Program Counter(vsepc)

use crate::register;

register!(
    Vsepc,              /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x241",   /* Read Instruction */
    "csrw 0x241, $0",   /* Write Instruction */
    {                   /* Register Field */
        VSEPC       OFFSET(0)  NUMBITS(64) []
    }
);
