//! Machine Trap-Vector Base-Address (mtvec)

use crate::register;

register!(
    Mtvec,              /* Register Name */
    u64,                /* Register Size */
    "csrr $0, 0x305",   /* Read Instruction */
    "csrw 0x305, $0",   /* Write Instruction */
    {                   /* Register Field */
        MODE       OFFSET(0)  NUMBITS(2) [],
        BASE       OFFSET(2)  NUMBITS(62) []
    }
);
