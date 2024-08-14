//! Supervisor Trap-Vector Base-Address (stvec)

use crate::register;

register!(
    Stvec,              /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x105",   /* Read Instruction */
    "csrw 0x105, $0",   /* Write Instruction */
    {                   /* Register Field */
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    }
);
